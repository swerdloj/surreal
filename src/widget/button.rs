use crate::state::State;
use crate::rectangle::Rectangle;
use crate::font::TextRenderer;

use crate::application::{gpu, RenderContext};

use std::cell::RefMut;

use sdl2::event::Event;

use super::Widget;

pub struct Button {
    id: &'static str,
    rect: Rectangle,
    on_click: Option<Box<dyn FnMut(RefMut<State>)>>,
}

impl Button {
    pub fn new(id: &'static str) -> Self {
        Button {
            id,
            rect: Rectangle::new(),
            on_click: None,
        }
    }

    pub fn on_click<F: FnMut(RefMut<State>) + 'static>(mut self, cb: F) -> Self {
        self.on_click = Some(Box::new(cb));
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

    fn render(&self, render_context: &mut RenderContext, gpu: &mut gpu, text_renderer: &mut TextRenderer) {
        
    }
}

impl crate::IntoViewElement for Button {
    fn into_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}