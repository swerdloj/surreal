use crate::view_element::*;

use super::Widget;

#[derive(IntoViewElement)]
#[kind(Widget)]
pub struct Text {
    id: &'static str,
    text: String,
    font: String,
    scale: f32,
    color: Option<crate::Color>,
    pub bounds: crate::bounding_rect::BoundingRect,

    section: Option<glyph_brush::OwnedSection>,
}

impl Text {
    pub fn new(id: &'static str) -> Self {
        Text {
            id,
            text: String::new(),
            font: String::from(""),
            scale: 16.0,
            color: None,
            bounds: crate::bounding_rect::BoundingRect::new(),

            section: None,
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
        self.color = Some(color);
        self
    }
}

impl Widget for Text {
    fn id(&self) -> &'static str {
        self.id
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.bounds.x += dx;
        self.bounds.y += dy;

        if let Some(section) = &mut self.section {
            section.screen_position = (self.bounds.x as f32, self.bounds.y as f32);
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        self.bounds.x = x;
        self.bounds.y = y;

        if let Some(section) = &mut self.section {
            section.screen_position = (x as f32, y as f32);
        }
    }

    fn init(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme) {       
        // 1. Create section
        let color = if let Some(color) = &self.color {
            color
        } else {
            &theme.colors.text
        };

        let text = wgpu_glyph::Text::new(&self.text)
            .with_scale(self.scale)
            .with_color(color.as_array())
            .with_font_id(text_renderer.get_font_id(&self.font));
        
        let section = wgpu_glyph::Section {
            text: vec![
                text,
            ],
            ..wgpu_glyph::Section::default()
        };

        let (width, height) = text_renderer.get_section_bounds(&section);
        
        // 2. Set widget bounds
        self.bounds.width = width;
        self.bounds.height = height;

        // 3. Store section
        self.section = Some(section.to_owned());
    }

    fn render_size(&self, _theme: &crate::style::Theme) -> (u32, u32) {
        self.bounds.dimensions()
        // text_renderer.get_section_bounds(&self...)
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, _theme: &crate::style::Theme) {       
        if let Some(section) = &self.section {
            renderer.draw(crate::render::DrawCommand::Text(section));
        }
    }
}