use crate::state::State;
use crate::bounding_rect::BoundingRect;

pub struct ScrollBar<Msg> {
    id: &'static str,
    bounds: BoundingRect,
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
            bounds: BoundingRect::new(),
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
}

impl<Msg: crate::EmptyMessage> super::Widget<Msg> for ScrollBar<Msg> {
    fn id(&self) -> &'static str {
        self.id
    }

    // TODO: Listen for scroll wheel
    fn handle_event(&mut self, event: &crate::event::ApplicationEvent, _state: std::cell::RefMut<crate::state::State>, _message_queue: &mut crate::MessageQueue<Msg>) -> crate::EventResponse {
        match event {
            crate::event::ApplicationEvent::MouseMotion { position } => {
                if self.is_held_down {
                    todo!()
                }

                crate::EventResponse::None
            }

            crate::event::ApplicationEvent::MouseButton { state, button, position } => {
                todo!()
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