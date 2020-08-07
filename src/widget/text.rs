use super::Widget;

use crate::application::{gpu, RenderTarget};
use crate::font::TextRenderer;

pub struct Text {
    id: &'static str,
    text: String,
    font: String,
    scale: f32,
    color: crate::Color,
    position: (i32, i32),
}

impl Text {
    pub fn new(id: &'static str) -> Self {
        Text {
            id,
            text: String::new(),
            font: String::from(""),
            scale: 16.0,
            color: crate::Color::WHITE,
            position: (0, 0),
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_owned();
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    
    pub fn font(mut self, font: &str) -> Self {
        self.font = font.to_owned();
        self
    }

    pub fn color(mut self, color: crate::Color) -> Self {
        self.color = color;
        self
    }
}

impl Widget for Text {
    fn id(&self) -> &'static str {
        self.id
    }

    fn render(&self, render_target: &mut RenderTarget, gpu: &mut gpu, text_renderer: &mut TextRenderer) {
        let font_id = if self.font == "" {
            text_renderer.get_font_id("default")
        } else {  
            text_renderer.get_font_id(&self.font)
        };
        
        let text = wgpu_glyph::Text::new(&self.text)
        .with_scale(self.scale)
        .with_color(self.color.as_array())
        .with_font_id(font_id);
        
        // TODO: Use Section instead of a String and variables
        // this would also allow users to have mutli-colored text,
        // bold/italic words, different sizes, etc.
        // Consider implementing text formatting like markdown-style
        // to make this easy for users 
        let section = wgpu_glyph::Section {
            screen_position: (self.position.0 as f32, self.position.1 as f32),
            text: vec![
                text,
            ],
            ..wgpu_glyph::Section::default()
        };
        
        text_renderer.render_section(
            gpu, 
            render_target.frame, 
            render_target.width, 
            render_target.height, 
            section,
        );
    }
}

impl crate::IntoViewElement for Text {
    fn into_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}