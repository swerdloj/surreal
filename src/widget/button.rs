use crate::state::State;
use crate::bounding_rect::BoundingRect;
use crate::view_element::*;

use std::cell::RefMut;

use super::Widget;

pub struct Button<Msg> {
    id: &'static str,
    bounds: BoundingRect,
    text: Option<super::Text<Msg>>,
    on_click: Option<Box<dyn FnMut(RefMut<State>) -> Msg>>,
    color: Option<crate::Color>,
    roundness: f32,

    // Register click only when mouse-down *and* mouse-up occur within bounds
    mouse_down_in_bounds: bool,

    should_resize: bool,
}

impl<Msg> Button<Msg> {
    pub fn new(id: &'static str) -> Self {
        let mut bounds = BoundingRect::new();

        // TODO: Defaults should be from theme & button contents
        bounds.width = 150;
        bounds.height = 75;

        Button {
            id,
            bounds,
            text: None,
            on_click: None,
            color: None,
            // Negative -> unset
            roundness: -1.0,
            mouse_down_in_bounds: false,
            should_resize: false,
        }
    }

    pub fn on_click<F: FnMut(RefMut<State>) -> Msg + 'static>(mut self, cb: F) -> Self {
        self.on_click = Some(Box::new(cb));
        self
    }

    pub fn text(mut self, text: super::Text<Msg>) -> Self {
        self.text = Some(text);
        self
    }

    pub fn color(mut self, color: crate::Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn roundness(mut self, roundness: f32) -> Self {
        if roundness < 0.0 || roundness > 100.0 {
            panic!("Roundness must be between 0 and 100 (percent). `{}` got `{}`", self.id, roundness);
        }

        self.roundness = roundness;
        self
    }
}

impl<Msg: EmptyMessage> Widget<Msg> for Button<Msg> where Msg: 'static {
    fn id(&self) -> &'static str {
        self.id
    }

    fn should_resize(&mut self) -> &mut bool {
        &mut self.should_resize
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.bounds.x += dx;
        self.bounds.y += dy;

        if let Some(text) = &mut self.text {
            text.translate(dx, dy);
        }
    }

    fn init(&mut self, renderer: &mut crate::render::Renderer, theme: &crate::style::Theme) {
        if self.roundness < 0.0 {
            self.roundness = theme.widget_styles.buttons.roundness;
        }

        if self.color.is_none() {
            self.color = Some(theme.colors.primary);
        }
        
        // TODO: Adjust the button's size according to text (if text is too big)
        if let Some(text) = &mut self.text {
            text.init(renderer, theme);
        }
    }

    fn handle_event(&mut self, event: &crate::event::ApplicationEvent, state: RefMut<State>, messages: &mut crate::MessageQueue<Msg>) -> crate::EventResponse {
        use crate::event::*;
        
        match event {
            ApplicationEvent::MouseButton { state: ButtonState::Pressed, button: MouseButton::Left, position: (x, y) } => {
                if self.bounds.contains(*x, *y) {
                    self.mouse_down_in_bounds = true;
                    return crate::EventResponse::Consume;
                }
            }
            
            ApplicationEvent::MouseButton { state: ButtonState::Released, button: MouseButton::Left, position: (x, y) } => {
                if self.mouse_down_in_bounds && self.bounds.contains(*x, *y) {
                    if let Some(on_click) = &mut self.on_click {
                        messages.push((on_click)(state));
                    }
                    self.mouse_down_in_bounds = false;
                    return crate::EventResponse::Consume;
                }
                
                self.mouse_down_in_bounds = false;
            }
            
            _ => {}
        }

        crate::EventResponse::None
    }

    fn place(&mut self, x: i32, y: i32) {
        self.bounds.x = x;
        self.bounds.y = y;

        if let Some(text) = &mut self.text {
            // TODO: Allow user to choose text's allignment
            let (text_width, text_height) = text.bounds.dimensions();
            text.place(x, y);

            // FIXME: `place` doesn't seem like the proper location for this
            text.translate(
                (self.bounds.width / 2) as i32 - (text_width / 2) as i32, 
                (self.bounds.height / 2) as i32 - (text_height / 2) as i32
            );
        }
    }

    fn render_size(&self, _theme: &crate::style::Theme) -> (u32, u32) {        
        // TODO: Account for text size
        (self.bounds.width, self.bounds.height)
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {        
        // TODO: Renderer can create draw commands using just the bounding_rect + style

        renderer.draw(crate::render::DrawCommand::RoundedRect {
            top_left: self.bounds.top_left(),
            width: self.bounds.width,
            height: self.bounds.height,
            roundness_percent: self.roundness,
            color: self.color.unwrap(),
        });

        if let Some(text) = &self.text {
            text.render(renderer, theme);
        }
    }
}