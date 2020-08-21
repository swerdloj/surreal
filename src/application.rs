use crate::view::View;

use sdl2::event::{Event, WindowEvent};
use wgpu::*;

struct sdl {
    pub context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
}

struct gpu {
    render_surface: Surface,
    adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    sc_desc: SwapChainDescriptor,
    pub swap_chain: SwapChain,
}

fn init_sdl2(title: &str, width: u32, height: u32) -> sdl {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();

    let mut window = video_subsystem.window(title, width, height)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // NOTE: SwapChain panics when a surface dimensions is 1
    // Resizing a window to be that small doesn't even make sense anyway
    // TODO: Minumum size should be equal to a title-bar once implemented
    // Title-bar contains the minimize, maximize, and exit buttons
    window.set_minimum_size(10, 10).unwrap();

    sdl {
        context: sdl2_context,
        video_subsystem,
        window,
    }
}

async fn init_wgpu(window: &sdl2::video::Window) -> gpu {
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
            features: Features::default(),
            limits: Limits::default(),
            shader_validation: true,
        }, 
        None,
    ).await.unwrap();

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

pub struct Application {
    sdl: sdl,
    gpu: gpu,

    timer: crate::timing::Timer,

    fonts: Option<crate::IncludedFonts>,
    global_theme: crate::style::Theme,

    fit_window_to_view: bool,
    is_minimized: bool,
}

impl Application {
    // TODO: Set options using a type like ApplicationSettings
    // Then, tweak fit_window_to_view so window sizing is most convenient for users
    pub fn new(title: &str, width: u32, height: u32, fonts: crate::IncludedFonts) -> Self {
        let sdl = init_sdl2(title, width, height);
        let gpu = futures::executor::block_on(
            init_wgpu(&sdl.window)
        );

        let timer = crate::timing::Timer::from_sdl2_context(&sdl.context);

        Application {
            sdl,
            gpu,
            timer,
            fonts: Some(fonts),
            global_theme: crate::style::DEFAULT_THEME,

            fit_window_to_view: false,
            is_minimized: false,
        }
    }

    // TODO: Integrate image library
    pub fn with_icon(mut self, icon: ()) -> Self {
        todo!();
    }

    pub fn with_global_theme(mut self, theme: crate::style::Theme) -> Self {
        self.global_theme = theme;
        self
    }

    pub fn fit_window_to_view(mut self, fit: bool) -> Self {
        self.fit_window_to_view = fit;
        self
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

    fn resize_swap_chain(&mut self, width: u32, height: u32) {
        self.gpu.sc_desc.width = width;
        self.gpu.sc_desc.height = height;

        self.gpu.swap_chain = self.gpu.device.create_swap_chain(&self.gpu.render_surface, &self.gpu.sc_desc);
    }

    pub fn run(&mut self, view: &mut dyn View) {
        let mut event_pump = self.sdl.context.event_pump().unwrap();
        
        let mut text_renderer = crate::render::font::TextRenderer::from_fonts(
            self.take_fonts(), 
            &self.gpu.device, 
            crate::TEXTURE_FORMAT
        );

        view.layout(&mut text_renderer, &self.global_theme);

        // TODO: Account for when the view changes
        if self.fit_window_to_view {
            let (width, height) = (view.render_width(), view.render_height());
            println!("Resizing window to view dimensions: {}x{}", width, height);
            self.sdl.window.set_size(width, height).unwrap();
            self.resize_swap_chain(width, height);
        }

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

                        self.resize_swap_chain(width as u32, height as u32);
                    }

                    Event::Window { win_event: WindowEvent::Minimized, .. } => {
                        println!("Minimizing");
                        self.is_minimized = true;
                    }

                    Event::Window { win_event: WindowEvent::Restored, .. } => {
                        println!("Restored");
                        self.is_minimized = false;
                    }

                    _ => {
                        // println!("Unhandled event: {:?}", event);
                    }
                }
            }

            // FIXME: wgpu panics at "Outdated" when the render surface changes (on window minimize)
            // This solves the issue, but I feel like there is a better solution
            if !self.is_minimized {
                self.render_view(&mut renderer, view);
            }
            
            let dt = self.timer.tick();
            crate::timing::Timer::await_fps(60, dt, 5);
        }
    }

    fn render_view(&mut self, renderer: &mut crate::render::Renderer, view: &mut dyn View) {
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
                        load: LoadOp::Clear(crate::Color::AUBERGINE.into()),
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: None,
        });

        // Bundle the renderer and context for user-side simplicity
        let mut renderer_with_context = crate::render::ContextualRenderer {
            renderer,
            device: &self.gpu.device,
            queue: &self.gpu.queue,
            target: &frame.output.view,
            encoder: &mut encoder,
            window_dimensions: (self.gpu.sc_desc.width, self.gpu.sc_desc.height),
        };

        // Render the entire view
        view.render(&mut renderer_with_context, &self.global_theme);

        // Does everything requested by the ContextualRenderer
        self.gpu.queue.submit(Some(encoder.finish()));
    }
}