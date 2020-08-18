use crate::state::State;
use crate::bounding_rect::BoundingRect;

use std::cell::RefMut;

use sdl2::event::Event;

use super::Widget;

pub struct Button {
    id: &'static str,
    bounds: BoundingRect,
    on_click: Option<Box<dyn FnMut(RefMut<State>)>>,
    // TEMP: Theme will handle this? Allow per-item theming?
    color: crate::Color,
}

impl Button {
    pub fn new(id: &'static str) -> Self {
        Button {
            id,
            bounds: BoundingRect::new(),
            on_click: None,
            color: crate::Color::WHITE,
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
        self.color = color;
        self
    }
}

impl Widget for Button {
    fn id(&self) -> &'static str {
        self.id
    }

    fn handle_event(&mut self, event: &Event) -> crate::EventResponse {
        match event {
            Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {
                crate::EventResponse::Consume
            }
            _ => crate::EventResponse::None,
        }
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer) {
        renderer.draw( crate::render::DrawCommand::Rect {
            top_left: self.bounds.top_left(),
            width: self.bounds.width,
            height: self.bounds.height,
            color: self.color,
        });
    }
}

impl crate::IntoViewElement for Button {
    fn into_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}