use crate::view::View;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::PhysicalSize,
};

use wgpu::*;

struct WindowSystem {
    event_loop: Option<EventLoop<()>>,
    window: winit::window::Window,
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
    pub allow_scrollbars: bool,
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
            allow_scrollbars: false,
            use_vsync: false,
            target_fps: 60,
        }
    }
}

pub struct Application {
    window_system: WindowSystem,
    gpu: GraphicsDevice,

    timer: crate::timing::Timer,

    renderer: crate::render::Renderer,
    target_fps: u64,

    global_theme: crate::style::Theme,

    fit_window_to_view: bool,
    is_resizable: bool,
    allows_scrollbars: bool,
    is_minimized: bool,
}

impl Application {
    pub fn new(settings: ApplicationSettings) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(settings.title)
            .with_inner_size(PhysicalSize::new(settings.width, settings.height))
            .with_resizable(settings.resizable)
            .build(&event_loop)
            .unwrap();
        
        let gpu = futures::executor::block_on(
            init_wgpu(&window, settings.use_vsync)
        );

        let timer = crate::timing::Timer::new();

        let renderer = crate::render::Renderer::new(
            &gpu.device, 
            &gpu.queue, 
            settings.fonts, 
            settings.images
        );

        let window_system = WindowSystem {
            event_loop: Some(event_loop),
            window,  
        };

        Application {
            window_system,
            gpu,
            timer,
            renderer,
            target_fps: settings.target_fps,
            global_theme: crate::style::DEFAULT_THEME,

            fit_window_to_view: settings.fit_window_to_view,
            is_resizable: settings.resizable,
            allows_scrollbars: settings.allow_scrollbars,
            is_minimized: false,
        }
    }

    fn resize_swap_chain(&mut self, width: u32, height: u32) {
        self.gpu.sc_desc.width = width;
        self.gpu.sc_desc.height = height;

        self.gpu.swap_chain = self.gpu.device.create_swap_chain(&self.gpu.render_surface, &self.gpu.sc_desc);
    }

    /// Runs the application with the given view. Targets web if `target_arch` is wasm32
    pub fn run<Msg: crate::EmptyMessage + 'static>(&mut self, view: &mut dyn View<Msg>) {
        #[cfg(not(target_arch = "wasm32"))]
        self.run_(view);

        #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().unwrap();
            
            // TEMP: Remove this once confirmed working
            log::info!("Test");

            use winit::platform::web::WindowExtWebSys;
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(self.window_system.window.canvas()))
                        .ok()
                })
                .expect("Couldn't append canvas to document body");

            self.run_(view);
        }
    }

    fn run_<Msg: crate::EmptyMessage + 'static>(&mut self, view: &mut dyn View<Msg>) {
        // FIXME: winit requires static lifetimes, but these aren't static.
        //        I can't take ownership of `self` here for the same reason (static closure requires `move`)
        //        Furthermore, I can't use `run_return` because this needs to work on the web too
        let this = unsafe { std::mem::transmute::<_, &'static mut Application>(self) };
        let view = unsafe { std::mem::transmute::<_, &'static mut dyn View<Msg>>(view) };
        
        let mut message_queue = crate::MessageQueue::new();

        view._init(&mut this.renderer, &this.global_theme, true);
        view.layout(&mut this.renderer, &this.global_theme, (this.gpu.sc_desc.width, this.gpu.sc_desc.height), true);
        
        {
            let (width, height) = view.render_size();
            
            // TODO: Account for when the view changes
            if this.fit_window_to_view {
                println!("Resizing window to view dimensions: {}x{}", width, height);
                this.window_system.window.set_inner_size(winit::dpi::LogicalSize::new(width, height));
                this.resize_swap_chain(width, height);
            }

            // FIXME: This needs to be updated when views become dynamic
            if this.is_resizable && !this.allows_scrollbars {
                this.window_system.window.set_min_inner_size(Some(winit::dpi::LogicalSize::new(width, height)));
            }
        }

        // FIXME: Window dimensions depend on the view size, but view size depends on window dimensions, so this happens twice
        view.layout(&mut this.renderer, &this.global_theme, (this.gpu.sc_desc.width, this.gpu.sc_desc.height), true);

        this.timer.start();

        #[cfg(feature = "frame-time")]
        let mut frame_time_accumulator: u128 = 0;
        #[cfg(feature = "frame-time")]
        let mut num_frames: u64 = 0;
        
        // Always render the first frame
        let mut should_render = true;
        let mut should_resize = false;

        let mut mouse_position: (i32, i32) = (0, 0);

        let event_loop = this.window_system.event_loop.take().unwrap();

        // Main loop
        event_loop.run(move |event, _, control_flow| {
            // TODO: Figure out the ideal usage of control_flow
            // *control_flow = ControlFlow::WaitUntil(std::time::Instant::);
            *control_flow = ControlFlow::Wait;

            let mut application_event = crate::event::ApplicationEvent::None;

            // Default event handlers
            match event {
                // Window close
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    #[cfg(feature = "frame-time")]
                    println!("Average frame time: {}ms", frame_time_accumulator as f64 / num_frames as f64);
                    
                    println!("Exiting main loop...");
                    *control_flow = ControlFlow::Exit;
                }

                // Window resize
                Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                    let (width, height) = (size.width, size.height);
                    // println!("Window resized to {}x{}", &width, &height);

                    // FIXME: winit does not have min/max events
                    if width == 0 {
                        this.is_minimized = true;
                    } else {
                        this.is_minimized = false;

                        this.resize_swap_chain(width, height);
                        // Centered views need this
                        should_resize = true;
                    }
                }

                // Touch -> Mouse
                // TODO: Flesh this out
                Event::WindowEvent { event: WindowEvent::Touch(touch), .. } => {
                    application_event = match touch.phase {
                        winit::event::TouchPhase::Started => crate::event::ApplicationEvent::MouseButton {
                            state: crate::event::ButtonState::Pressed,
                            button: crate::event::MouseButton::Left,
                            position: touch.location.into(),
                        },
                        
                        winit::event::TouchPhase::Ended => crate::event::ApplicationEvent::MouseButton {
                            state: crate::event::ButtonState::Released,
                            button: crate::event::MouseButton::Left,
                            position: touch.location.into(),
                        },
                        
                        winit::event::TouchPhase::Moved => {
                            let position = touch.location.into();
                            let event = crate::event::ApplicationEvent::MouseMotion {
                                position,
                                relative_change: (position.0 - mouse_position.0, position.1 - mouse_position.1),
                            };
                            mouse_position = position;
                            event
                        }, 
                        
                        winit::event::TouchPhase::Cancelled => todo!("touch-cancel"), // TODO: what does this mean?
                    };
                }

                // Mouse motion
                Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                    let physical = position.to_logical(this.window_system.window.scale_factor());

                    application_event = crate::event::ApplicationEvent::MouseMotion {
                        position: (physical.x, physical.y),
                        relative_change: (physical.x - mouse_position.0, physical.y - mouse_position.1),
                    };

                    mouse_position = (physical.x, physical.y);
                }

                // Mouse click
                Event::WindowEvent { event: WindowEvent::MouseInput { button, state, .. }, .. } => {
                    let button = match button {
                        winit::event::MouseButton::Left => crate::event::MouseButton::Left,
                        winit::event::MouseButton::Right => crate::event::MouseButton::Right,
                        winit::event::MouseButton::Middle => crate::event::MouseButton::Middle,
                        winit::event::MouseButton::Other(n) => crate::event::MouseButton::Other(n),
                    };

                    let state = match state {
                        winit::event::ElementState::Pressed => crate::event::ButtonState::Pressed,
                        winit::event::ElementState::Released => crate::event::ButtonState::Released,
                    };

                    application_event = crate::event::ApplicationEvent::MouseButton {
                        state,
                        button,
                        position: mouse_position,
                    };
                }

                // Draw to window
                Event::RedrawRequested(_) => {
                    // Time since last frame
                    let _dt = this.timer.tick();

                    #[cfg(feature = "frame-time")]
                    let start = std::time::Instant::now();
        
                    if should_resize {
                        view._init(&mut this.renderer, &this.global_theme, false);
                        view.layout(&mut this.renderer, &this.global_theme, (this.gpu.sc_desc.width, this.gpu.sc_desc.height), true);
                        // render the updated view
                        should_render = true;
                    }
                    
                    // If nothing is happening, there is no need to render at 60FPS
                    if !this.is_minimized && should_render {
                        this.render_view(view);
                        should_render = false;
                        should_resize = false;
                    }

                    #[cfg(feature = "frame-time")] {
                        frame_time_accumulator += start.elapsed().as_millis();
                        num_frames += 1;
                    }
                }

                _ => {
                    // println!("Unhandled event: {:?}", event);
                }
            } // match event

            if !application_event.is_none() {
                should_render |= view.propogate_event(&application_event, &mut message_queue);
                
                // TODO: Should this happen only once during RedrawRequested?
                for message in message_queue.drain() {
                    // FIXME: I can't make `call_hook` part of `View`
                    crate::view::call_hook(view, &message);
                    // If view resized, render the view
                    should_resize |= view.propogate_message(&message);
                }
                
                // Only render if there is a reason to
                if !this.is_minimized && (should_resize || should_render) {
                    this.window_system.window.request_redraw();
                }
                
                // FIXME: Is this usage correct with winit?
                // this.timer.await_fps(this.target_fps, 5);
            }
        });        
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


async fn init_wgpu(window: &winit::window::Window, use_vsync: bool) -> GraphicsDevice {
    let (width, height) = window.inner_size().into();

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