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

pub struct Application {
    sdl: sdl,
    gpu: gpu,
}

impl Application {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl = Self::init_sdl2(title, width, height);
        let gpu = futures::executor::block_on(
            Self::init_wgpu(&sdl.window)
        );

        Application {
            sdl,
            gpu,
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
        
        // TEMP: This is here for testing purposes
        let mut test_text_renderer = crate::font::TextRenderer::from_fonts(
            include_fonts! { 
                default => "../res/JetBrainsMono/JetBrainsMono-Medium.ttf",
            }, 
            &self.gpu.device,
            TextureFormat::Bgra8UnormSrgb
        );

        'main_loop: loop {
            // FIXME: Program crashes on resize unless this is scoped
            // Probably caused by having render_target alive when creating new swap chain
            {
                let render_target = &self.gpu.swap_chain.get_next_texture().unwrap().view;
                let (width, height) = self.sdl.window.size();
                test_text_renderer.render_text(&mut self.gpu, &render_target, width, height, "This is a test");
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'main_loop;
                    }

                    Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                        self.gpu.sc_desc.width = width as u32;
                        self.gpu.sc_desc.height = height as u32;

                        self.gpu.swap_chain = self.gpu.device.create_swap_chain(&self.gpu.render_surface, &self.gpu.sc_desc);
                    }

                    _ => {
                        // println!("Unhandled event: {:?}", event);
                    }
                }
            }

            // TEMP: Force 60 FPS
            std::thread::sleep(std::time::Duration::from_millis((1.0/60.0) as u64 * 1000));
        }
    }
}