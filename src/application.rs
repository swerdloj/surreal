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

/// Texture for drawing to window
pub struct RenderTarget<'sc_output> {
    pub frame: &'sc_output TextureView,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
}

pub struct Application {
    sdl: sdl,
    gpu: gpu,

    text_renderer: crate::font::TextRenderer,
}

impl Application {
    // TODO: Accept theme here (which would include the fonts)
    pub fn new(title: &str, width: u32, height: u32, fonts: Vec<(&'static str, wgpu_glyph::ab_glyph::FontArc)>) -> Self {
        let sdl = Self::init_sdl2(title, width, height);
        let gpu = futures::executor::block_on(
            Self::init_wgpu(&sdl.window)
        );

        let text_renderer = crate::font::TextRenderer::from_fonts(fonts, &gpu.device, TextureFormat::Bgra8UnormSrgb);

        Application {
            sdl,
            gpu,
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
            format: TextureFormat::Bgra8UnormSrgb,
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
                label: Some("clear_frame"),
            });
            encoder.begin_render_pass(&RenderPassDescriptor {
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
            // FIXME: encoder should be re-used in `render_text` (but this is temporary)
            // Consider storing it in RenderTarget
            self.gpu.queue.submit(&[encoder.finish()]);

            let mut render_context = RenderTarget {
                frame: &render_target.view,
                width: window_width,
                height: window_height,
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
            
            // TEMP: Force 60 FPS
            std::thread::sleep(std::time::Duration::from_millis((1.0/60.0) as u64 * 1000));
        }
    }
}