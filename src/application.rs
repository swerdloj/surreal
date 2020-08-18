use crate::view::View;

use sdl2::event::{Event, WindowEvent};
use wgpu::*;

pub struct sdl {
    pub context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
}

pub struct gpu {
    render_surface: Surface,
    adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    sc_desc: SwapChainDescriptor,
    pub swap_chain: SwapChain,
}

// TODO: Replace this with render::Renderer
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

    timer: crate::timing::Timer,

    fonts: Option<crate::IncludedFonts>,
}

impl Application {
    // TODO: Accept theme here (which would include the fonts)
    pub fn new(title: &str, width: u32, height: u32, fonts: crate::IncludedFonts) -> Self {
        let sdl = Self::init_sdl2(title, width, height);
        let gpu = futures::executor::block_on(
            Self::init_wgpu(&sdl.window)
        );

        let timer = crate::timing::Timer::from_sdl2_context(&sdl.context);

        Application {
            sdl,
            gpu,
            timer,
            fonts: Some(fonts),
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

    // FIXME: This isn't the best solution, but I don't want to pass fonts into `run`
    fn take_fonts(&mut self) -> crate::IncludedFonts {
        if let Some(_fonts) = &self.fonts {} else {
            panic!("Fonts were already moved")
        }

        let mut swap = None;
        std::mem::swap(&mut self.fonts, &mut swap);

        swap.unwrap()
    }

    pub fn run(&mut self, view: &mut dyn View) {
        let mut event_pump = self.sdl.context.event_pump().unwrap();
        
        let text_renderer = crate::render::font::TextRenderer::from_fonts(
            self.take_fonts(), 
            &self.gpu.device, 
            crate::TEXTURE_FORMAT
        );

        let mut renderer = crate::render::Renderer::new(
            &self.gpu.device,
            text_renderer,
        );

        self.timer.start();
        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        println!("Exiting main loop...");
                        break 'main_loop;
                    }

                    Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                        println!("Window resized to {}x{}", &width, &height);

                        self.gpu.sc_desc.width = width as u32;
                        self.gpu.sc_desc.height = height as u32;

                        self.gpu.swap_chain = self.gpu.device.create_swap_chain(&self.gpu.render_surface, &self.gpu.sc_desc);
                    }

                    _ => {
                        // println!("Unhandled event: {:?}", event);
                    }
                }
            }

            self.render_view(&mut renderer, view);
            
            let dt = self.timer.tick();
            crate::timing::Timer::await_fps(60, dt, 5);
        }
    }

    fn render_view(&mut self, renderer: &mut crate::render::Renderer, view: &mut dyn View) {
        let render_target = self.gpu.swap_chain.get_next_texture().unwrap();
                
        let mut encoder = self.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        // Initial frame clear
        let _ = encoder.begin_render_pass(&RenderPassDescriptor {
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

        // All information needed by the renderer
        let ctx = crate::render::RenderContext {
            device: &self.gpu.device,
            target: &render_target.view,
            encoder: &mut encoder,
        };

        // Bundle the renderer and context for user-side simplicity
        let mut renderer_with_context = crate::render::ContextualRenderer {
            ctx,
            renderer,
            window_dimensions: (self.gpu.sc_desc.width, self.gpu.sc_desc.height),
        };

        // Render the view
        for element in view.children() {
            use crate::ViewElement::*;
            match element {
                View(_view) => {
                    // TODO: Render its contents
                }
                
                Widget(widget) => {
                    widget.render(&mut renderer_with_context);
                }
                
                TEMP_State(_state) => {
                    // This will be removed eventually
                }
            }
        }

        // Does everything requested by the ContextualRenderer
        self.gpu.queue.submit(&[encoder.finish()]);
    }
}