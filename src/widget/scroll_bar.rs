use crate::state::State;
use crate::bounding_rect::BoundingRect;

pub struct ScrollBar<Msg> {
    id: &'static str,
    container_bounds: BoundingRect,
    slider_bounds: BoundingRect,
    orientation: crate::Orientation,
    // TODO: Pass in scroll information here
    on_scroll: Option<Box<dyn FnMut(std::cell::RefMut<State>) -> Msg>>,
    // Scroller
    slider_roundness: Option<f32>,
    // Scroll container
    background_roundness: Option<f32>,
    // Whether the slider has a container
    has_background: bool,

    background_color: Option<crate::Color>,
    slider_color: Option<crate::Color>,

    is_held_down: bool,

    should_resize: bool,
}

impl<Msg> ScrollBar<Msg> {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            container_bounds: BoundingRect::new(),
            slider_bounds: BoundingRect::new(),
            orientation: crate::Orientation::Vertical,
            on_scroll: None,
            slider_roundness: None,
            background_roundness: None,
            has_background: true,

            background_color: None,
            slider_color: None,

            is_held_down: false,
            should_resize: false,
        }
    }

    pub fn orientation(mut self, orientation: crate::Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn on_scroll<F: FnMut(std::cell::RefMut<State>) -> Msg + 'static>(mut self, on_scroll: F) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }

    fn scroll(&mut self, dx: i32, dy: i32) {
        match self.orientation {
            // use x
            crate::Orientation::Vertical => {
                todo!("scroll within container bounds")
            }
            // use y
            crate::Orientation::Horizontal => {
                todo!("scroll within container bounds")
            }
        }
    }
}

impl<Msg: crate::EmptyMessage> super::Widget<Msg> for ScrollBar<Msg> {
    fn id(&self) -> &'static str {
        self.id
    }

    // TODO: Listen for scroll wheel
    fn handle_event(&mut self, event: &crate::event::ApplicationEvent, _state: std::cell::RefMut<crate::state::State>, _message_queue: &mut crate::MessageQueue<Msg>) -> crate::EventResponse {
        use crate::event::*;
        
        match event {
            ApplicationEvent::MouseMotion { relative_change: (dx, dy), .. } => {
                if self.is_held_down {
                    self.scroll(*dx, *dy);
                }

                crate::EventResponse::None
            }

            ApplicationEvent::MouseButton { state: ButtonState::Pressed, button: MouseButton::Left, position: (x, y) } => {
                // TODO: Differentiate slider & container bounds.
                // Clicking on container should scroll to that point automatically.
                if self.slider_bounds.contains(*x, *y) {
                    self.is_held_down = true;
                    return crate::EventResponse::Consume;
                } else if self.container_bounds.contains(*x, *y) {
                    todo!("scroll to that point")
                }

                crate::EventResponse::None
            }

            ApplicationEvent::MouseButton { state: ButtonState::Released, button: MouseButton::Left, position: (x, y) } => {
                self.is_held_down = false;

                crate::EventResponse::None
            }

            _ => {
                crate::EventResponse::None
            }
        }
    }

    fn should_resize(&mut self) -> &mut bool {
        &mut self.should_resize
    }

    fn init(&mut self, _renderer: &mut crate::render::Renderer, theme: &crate::style::Theme) {
        if self.background_color.is_none() {
            self.background_color = Some(theme.colors.secondary);
        }

        if self.slider_color.is_none() {
            self.slider_color = Some(theme.colors.primary);
        }

        if self.slider_roundness.is_none() {
            self.slider_roundness = Some(theme.widget_styles.buttons.roundness);
        }

        if self.background_roundness.is_none() {
            self.slider_roundness = Some(theme.widget_styles.buttons.roundness);
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        todo!()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        todo!()
    }

    fn render_size(&self, theme: &crate::style::Theme) -> (u32, u32) {
        todo!()
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {
        todo!()
    }
}