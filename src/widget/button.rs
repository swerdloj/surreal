use crate::state::State;
use crate::rectangle::Rectangle;
use crate::font::TextRenderer;

use crate::application::{gpu, RenderTarget};

use std::cell::RefMut;

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

    fn render(&self, render_target: &mut RenderTarget, gpu: &mut gpu, text_renderer: &mut TextRenderer) {
        
    }
}

impl crate::IntoViewElement for Button {
    fn into_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}