use super::Widget;

use crate::application::{gpu, RenderTarget};
use crate::font::TextRenderer;

pub struct Text {
    id: &'static str,
    text: String,

    font: String,
    scale: f32,
    color: crate::Color,
}

impl Text {
    pub fn new(id: &'static str) -> Self {
        Text {
            id,
            text: String::new(),
            font: String::from(""),
            scale: 16.0,
            color: crate::Color::WHITE,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_owned();
        self
    }
    
    pub fn font(mut self, font: &str) -> Self {
        self.font = font.to_owned();
        self
    }
}

impl Widget for Text {
    fn id(&self) -> &'static str {
        self.id
    }

    fn render(&self, render_target: &mut RenderTarget, gpu: &mut gpu, text_renderer: &mut TextRenderer) {
        text_renderer.render_text(gpu, render_target.frame, render_target.width, render_target.height, &self.text);
    }
}

impl crate::IntoViewElement for Text {
    fn into_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}