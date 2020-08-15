use crate::view::View;

use sdl2::event::{Event, WindowEvent};
use wgpu::*;

pub struct sdl {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
}

pub struct gpu {
    render_surface: Surface,
    adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
}

// TODO: Make text rendering a part of RenderContext (e.g.: RenderContext::render_text())
/// Context for rendering to frame
pub struct RenderContext<'frame> {
    pub frame: &'frame TextureView,
    pub render_pass: RenderPass<'frame>,
    pub command_buffers: &'frame mut Vec<CommandBuffer>,

    pub frame_width: u32,
    pub frame_height: u32,
    pub format: TextureFormat,
}

pub struct Application {
    sdl: sdl,
    gpu: gpu,

    quad_bind_group_layout: BindGroupLayout,
    quad_render_pipeline: RenderPipeline,

    text_renderer: crate::font::TextRenderer,
}

impl Application {
    // TODO: Accept theme here (which would include the fonts)
    pub fn new(title: &str, width: u32, height: u32, fonts: Vec<(&'static str, wgpu_glyph::ab_glyph::FontArc)>) -> Self {
        let sdl = Self::init_sdl2(title, width, height);
        let gpu = futures::executor::block_on(
            Self::init_wgpu(&sdl.window)
        );

        let text_renderer = crate::font::TextRenderer::from_fonts(fonts, &gpu.device, crate::TEXTURE_FORMAT);

        let quad_bind_group_layout = crate::render::Quad::bind_group_layout(&gpu.device);
        let quad_render_pipeline = crate::render::Quad::create_render_pipeline(&gpu.device, &quad_bind_group_layout, crate::TEXTURE_FORMAT);

        Application {
            sdl,
            gpu,

            quad_bind_group_layout,
            quad_render_pipeline,

            text_renderer,
        }
    }

    fn init_sdl2(title: &str, width: u32, height: u32) -> sdl {
        let sdl2_context = sdl2::init().unwrap();
        let video_subsystem = sdl2_context.video().unwrap();

        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        sdl {
            context: sdl2_context,
            video_subsystem,
            window,
        }
    }

    async fn init_wgpu(window: &sdl2::video::Window) -> gpu {
        let (width, height) = window.size();
        let render_surface = Surface::create(window);

        let adapter = Adapter::request(&RequestAdapterOptions {
                // TODO: Allow user to choose GPU
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&render_surface),
            }, 
            BackendBit::PRIMARY,
        ).await.unwrap();

        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            extensions: Extensions {
                anisotropic_filtering: false,
            },
            limits: Limits::default(),
        }).await;

        let sc_desc = SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: crate::TEXTURE_FORMAT,
            width,
            height,
            // TODO: Allow user to toggle vsync
            present_mode: PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&render_surface, &sc_desc);

        gpu {
            render_surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
        }
    }

    pub fn run(mut self, view: &mut dyn View) {
        let mut event_pump = self.sdl.context.event_pump().unwrap();
        
        let (mut window_width, mut window_height) = self.sdl.window.size();
        
        // TEMP:
        let test_quad = crate::render::Quad::new(&self.gpu.device, &self.quad_bind_group_layout);

        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        println!("Exiting main loop...");
                        break 'main_loop;
                    }

                    Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                        window_width = width as u32;
                        window_height = height as u32;

                        self.gpu.sc_desc.width = window_width;
                        self.gpu.sc_desc.height = window_height;

                        self.gpu.swap_chain = self.gpu.device.create_swap_chain(&self.gpu.render_surface, &self.gpu.sc_desc);
                    }

                    _ => {
                        // println!("Unhandled event: {:?}", event);
                    }
                }
            }

            let render_target = self.gpu.swap_chain.get_next_texture().unwrap();
                
            // TEMP: Clear the frame
            let mut encoder = self.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("frame_encoder"),
            });

            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[
                    RenderPassColorAttachmentDescriptor {
                        attachment: &render_target.view,
                        resolve_target: None,
                        load_op: LoadOp::Clear,
                        store_op: StoreOp::Store,
                        clear_color: crate::Color::AUBERGINE.into(),
                    },
                ],
                depth_stencil_attachment: None,
            });

            // TEMP: Testing quad rendering
            render_pass.set_pipeline(&self.quad_render_pipeline);
            render_pass.set_bind_group(0, &test_quad.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &test_quad.vertex_buffer, 0, 0);
            render_pass.set_index_buffer(&test_quad.index_buffer, 0, 0);
            render_pass.draw_indexed(0..6, 0, 0..1);

            // NOTE: This must be declared here and stored as a reference
            // to allow render_context to be dropped before submitting the buffers.
            // In doing so, text can be rendered after clearing the frame.
            // FIXME: Actual draw order is lost -- text always goes on top layer
            let mut command_buffers = Vec::new();
            let mut render_context = RenderContext {
                frame: &render_target.view,
                render_pass,
                command_buffers: &mut command_buffers,

                frame_width: window_width,
                frame_height: window_height,
                format: TextureFormat::Rgba8UnormSrgb,
            };

            // Render the view
            for element in view.children() {
                use crate::ViewElement::*;
                match element {
                    View(_view) => {
                        // TODO: Render its contents
                    }

                    Widget(widget) => {
                        widget.render(&mut render_context, &mut self.gpu, &mut self.text_renderer);
                    }

                    TEMP_State(_state) => {
                        // This will be removed eventually
                    }
                }
            }

            drop(render_context);
            self.gpu.queue.submit(&[encoder.finish()]);
            self.gpu.queue.submit(&command_buffers);
            
            // TEMP: Force 60 FPS
            std::thread::sleep(std::time::Duration::from_millis((1.0/60.0) as u64 * 1000));
        }
    }
}