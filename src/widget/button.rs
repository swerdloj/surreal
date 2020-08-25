use crate::state::State;
use crate::bounding_rect::BoundingRect;

use std::cell::RefMut;

use sdl2::event::Event;

use super::Widget;

pub struct Button {
    id: &'static str,
    bounds: BoundingRect,
    on_click: Option<Box<dyn FnMut(RefMut<State>)>>,
    color: Option<crate::Color>,

    // Register click only when mouse-down *and* mouse-up occur within bounds
    mouse_down_in_bounds: bool,
}

impl Button {
    pub fn new(id: &'static str) -> Self {
        let mut bounds = BoundingRect::new();

        // TODO: Defaults should be from theme
        bounds.width = 150;
        bounds.height = 75;

        Button {
            id,
            bounds,
            on_click: None,
            color: None,
            mouse_down_in_bounds: false,
        }
    }

    pub fn on_click<F: FnMut(RefMut<State>) + 'static>(mut self, cb: F) -> Self {
        self.on_click = Some(Box::new(cb));
        self
    }

    // TEMP:
    pub fn bounds(mut self, bounds: BoundingRect) -> Self {
        self.bounds = bounds;
        self
    }

    pub fn color(mut self, color: crate::Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for Button {
    fn id(&self) -> &'static str {
        self.id
    }

    fn handle_event(&mut self, event: &Event, state: RefMut<State>) -> crate::EventResponse {
        match event {
            Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                if self.bounds.contains(*x, *y) {
                    self.mouse_down_in_bounds = true;
                }

                crate::EventResponse::Consume
            }

            Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                if self.mouse_down_in_bounds && self.bounds.contains(*x, *y) {
                    if let Some(cb) = &mut self.on_click {
                        (cb)(state);
                    }
                }
                
                self.mouse_down_in_bounds = false;
                crate::EventResponse::Consume
            }
            _ => crate::EventResponse::None,
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        self.bounds.x = x;
        self.bounds.y = y;
    }

    fn render_size(&self, _text_renderer: &mut crate::render::font::TextRenderer, _theme: &crate::style::Theme) -> (u32, u32) {
        (self.bounds.width, self.bounds.height)
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {        
        let color = if let Some(color) = self.color {
            color
        } else {
            theme.colors.primary
        };

        renderer.draw(crate::render::DrawCommand::Rect {
            top_left: self.bounds.top_left(),
            width: self.bounds.width,
            height: self.bounds.height,
            color,
        });
    }
}

impl crate::IntoViewElement for Button {
    fn into_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}