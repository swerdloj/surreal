use crate::view::View;

use sdl2::event::{Event, WindowEvent};
use wgpu::*;

struct WindowSystem {
    pub context: sdl2::Sdl,
    _video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
}

struct GraphicsDevice {
    render_surface: Surface,
    _adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    sc_desc: SwapChainDescriptor,
    pub swap_chain: SwapChain,
}

// TODO: An "ApplicationBuilder" might be needed for allowing more windowing options
pub struct ApplicationSettings {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub global_theme: crate::style::Theme,
    pub images: crate::widget::IncludedImages,
    // TODO: Reference the desired image resource
    pub app_icon: (),
    pub fonts: crate::render::font::IncludedFonts,
    pub fit_window_to_view: bool,
    pub resizable: bool,
    pub use_vsync: bool,
    pub target_fps: u64,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            title: "Surreal Application",
            width: 800,
            height: 600,
            global_theme: crate::style::DEFAULT_THEME,
            images: Vec::new(),
            app_icon: (),
            fonts: Vec::new(),
            fit_window_to_view: true,
            resizable: true,
            use_vsync: false,
            target_fps: 60,
        }
    }
}

pub struct Application {
    sdl: WindowSystem,
    gpu: GraphicsDevice,

    timer: crate::timing::Timer,

    renderer: crate::render::Renderer,
    target_fps: u64,

    global_theme: crate::style::Theme,

    fit_window_to_view: bool,
    is_resizable: bool,
    is_minimized: bool,
}

impl Application {
    pub fn new(settings: ApplicationSettings) -> Self {
        let sdl = init_sdl2(settings.title, settings.width, settings.height, settings.resizable);
        let gpu = futures::executor::block_on(
            init_wgpu(&sdl.window, settings.use_vsync)
        );

        let timer = crate::timing::Timer::new();

        let renderer = crate::render::Renderer::new(
            &gpu.device, 
            &gpu.queue, 
            settings.fonts, 
            settings.images
        );

        Application {
            sdl,
            gpu,
            timer,
            renderer,
            target_fps: settings.target_fps,
            global_theme: crate::style::DEFAULT_THEME,

            fit_window_to_view: settings.fit_window_to_view,
            is_resizable: settings.resizable,
            is_minimized: false,
        }
    }

    fn resize_swap_chain(&mut self, width: u32, height: u32) {
        self.gpu.sc_desc.width = width;
        self.gpu.sc_desc.height = height;

        self.gpu.swap_chain = self.gpu.device.create_swap_chain(&self.gpu.render_surface, &self.gpu.sc_desc);
    }

    pub fn run<Msg: crate::EmptyMessage + 'static>(&mut self, view: &mut dyn View<Msg>) {
        let mut event_pump = self.sdl.context.event_pump().unwrap();

        let mut message_queue = crate::MessageQueue::new();

        view._init(&mut self.renderer, &self.global_theme);
        view.layout(&mut self.renderer, &self.global_theme, (self.gpu.sc_desc.width, self.gpu.sc_desc.height), true);
        
        {
            let (width, height) = view.render_size();
            
            // TODO: Account for when the view changes
            if self.fit_window_to_view {
                println!("Resizing window to view dimensions: {}x{}", width, height);
                self.sdl.window.set_size(width, height).unwrap();
                self.resize_swap_chain(width, height);
            }

            // FIXME: This needs to be updated when views become dynamic
            if self.is_resizable {
                self.sdl.window.set_minimum_size(width, height).unwrap();
            }
        }

        // FIXME: Window dimensions depend on the view size, but view size depends on window dimensions, so this happens twice
        view.layout(&mut self.renderer, &self.global_theme, (self.gpu.sc_desc.width, self.gpu.sc_desc.height), true);

        self.timer.start();

        #[cfg(feature = "frame-time")]
        let mut frame_time_accumulator: u128 = 0;
        #[cfg(feature = "frame-time")]
        let mut num_frames: u64 = 0;
        
        // Always render the first frame
        let mut should_render = true;

        'main_loop: loop {
            // Time since last frame
            let _dt = self.timer.tick();

            #[cfg(feature = "frame-time")]
            let start = std::time::Instant::now();

            let mut should_resize = false;

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        #[cfg(feature = "frame-time")]
                        println!("Average frame time: {}ms", frame_time_accumulator as f64 / num_frames as f64);
                        println!("Exiting main loop...");
                        break 'main_loop;
                    }

                    Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                        println!("Window resized to {}x{}", &width, &height);

                        self.resize_swap_chain(width as u32, height as u32);
                        // Centered views need this
                        should_resize = true;
                    }

                    Event::Window { win_event: WindowEvent::Minimized, .. } => self.is_minimized = true,
                    Event::Window { win_event: WindowEvent::Restored, .. } => self.is_minimized = false,

                    _ => {
                        // println!("Unhandled event: {:?}", event);
                    }
                }

                view.propogate_event(&event, &mut message_queue);
            }
            
            
            for message in message_queue.drain() {
                // FIXME: I can't make `call_hook` part of `View`
                crate::view::call_hook(view, &message);
                // If view resized, render the view
                should_resize |= view.propogate_message(&message);
            }

            if should_resize {
                view._init(&mut self.renderer, &self.global_theme);
                view.layout(&mut self.renderer, &self.global_theme, (self.gpu.sc_desc.width, self.gpu.sc_desc.height), true);
                // render the updated view
                should_render = true;
            }
            
            // FIXME: wgpu panics at "Outdated" when the render surface changes (on window minimize)
            // This solves the issue, but I feel like there is a better solution
            if !self.is_minimized {
                // If nothing is happening, there is no need to render at 60FPS
                if should_render {
                    self.render_view(view);
                    should_render = false;
                }
            }
            
            #[cfg(feature = "frame-time")] {
                frame_time_accumulator += start.elapsed().as_millis();
                num_frames += 1;
            }

            self.timer.await_fps(self.target_fps, 5);
        }
    }

    fn render_view<Msg: crate::EmptyMessage + 'static>(&mut self, view: &mut dyn View<Msg>) {
        let frame = self.gpu.swap_chain.get_current_frame().unwrap();
                
        let mut encoder = self.gpu.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        // Initial frame clear
        let _ = encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[
                RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.global_theme.colors.background.into()),
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: None,
        });

        // Bundle the renderer and context for user-side simplicity
        let mut renderer_with_context = crate::render::ContextualRenderer {
            renderer: &mut self.renderer,
            device: &self.gpu.device,
            queue: &self.gpu.queue,
            target: &frame.output.view,
            encoder: &mut encoder,
            window_dimensions: (self.gpu.sc_desc.width, self.gpu.sc_desc.height),
        };

        // Render the entire view
        view.render(&mut renderer_with_context, &self.global_theme);

        // This function should be called only once per frame
        // This is so wgpu_glyph can cache the text, meaning this call should not be made inside `View`
        // Using individual draw calls per `Section` raises CPU usage from <1% to >5% (>22% in debug build)
        // NOTE: Placing this here satisfies the above, but sacrifices layering/ordering (see GitHub card)
        self.renderer.text_renderer.render_queue(&self.gpu.device, &frame.output.view, &mut encoder, self.gpu.sc_desc.width, self.gpu.sc_desc.height);

        // Does everything requested by the ContextualRenderer
        self.gpu.queue.submit(Some(encoder.finish()));
    }
}

fn init_sdl2(title: &str, width: u32, height: u32, resizable: bool) -> WindowSystem {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    // FIXME: Ugly solution
    let mut window = if resizable {
        video_subsystem.window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .unwrap()
    } else {
        video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .unwrap()
    };

    // NOTE: SwapChain panics when a surface dimensions is 1
    // Resizing a window to be that small doesn't even make sense anyway
    // TODO: Minumum size should be equal to a title-bar once implemented
    // Title-bar contains the minimize, maximize, and exit buttons
    window.set_minimum_size(10, 10).unwrap();

    WindowSystem {
        context: sdl2_context,
        _video_subsystem: video_subsystem,
        window,
    }
}

async fn init_wgpu(window: &sdl2::video::Window, use_vsync: bool) -> GraphicsDevice {
    let (width, height) = window.size();

    let instance = Instance::new(BackendBit::PRIMARY);

    let render_surface = unsafe { instance.create_surface(window) };

    let adapter = instance.request_adapter(&RequestAdapterOptions {
        // TODO: Allow user to choose GPU
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: Some(&render_surface),
    }).await.unwrap();

    let (device, queue) = adapter.request_device(
        &DeviceDescriptor {
            features: Features::empty(),
            limits: Limits::default(),
            shader_validation: false,
        }, 
        None,
    ).await.unwrap();

    let present_mode = if use_vsync {
        PresentMode::Fifo
    } else {
        // immediate gives lowest frame-times (no waiting period)
        PresentMode::Immediate 
    };

    let sc_desc = SwapChainDescriptor {
        usage: TextureUsage::OUTPUT_ATTACHMENT,
        format: crate::TEXTURE_FORMAT,
        width,
        height,
        present_mode,
    };

    let swap_chain = device.create_swap_chain(&render_surface, &sc_desc);

    GraphicsDevice {
        render_surface,
        _adapter: adapter,
        device,
        queue,
        sc_desc,
        swap_chain,
    }
}