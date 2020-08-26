use super::Widget;

pub struct Text {
    id: &'static str,
    text: String,
    font: String,
    scale: f32,
    color: Option<crate::Color>,
    bounds: crate::bounding_rect::BoundingRect,
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

    fn place(&mut self, x: i32, y: i32) {
        self.bounds.x = x;
        self.bounds.y = y;
    }

    fn render_size(&self, text_renderer: &mut crate::render::font::TextRenderer, _theme: &crate::style::Theme) -> (u32, u32) {
        // TODO: Same as in `render`: section can be saved for later use

        let text = wgpu_glyph::Text::new(&self.text)
            .with_scale(self.scale)
            .with_font_id(text_renderer.get_font_id(&self.font));
        
        let section = wgpu_glyph::Section {
            text: vec![
                text,
            ],
            ..wgpu_glyph::Section::default()
        };

        text_renderer.get_section_bounds(section)
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {
        // Reference to avoid cloning
        let color = if let Some(color) = &self.color {
            color
        } else {
            &theme.colors.text
        };
        
        let text = wgpu_glyph::Text::new(&self.text)
            .with_scale(self.scale)
            .with_color(color.as_array())
            .with_font_id(renderer.get_font_id(&self.font));
        
        // TODO: Use Section instead of a String and variables
        // this would also allow users to have mutli-colored text,
        // bold/italic words, different sizes, etc.
        // Consider implementing text formatting like markdown-style
        // to make this easy for users 
        // TODO: Store this. It doesn't need to be re-created each render
        let section = wgpu_glyph::Section {
            screen_position: (self.bounds.x as f32, self.bounds.y as f32),
            text: vec![
                text,
            ],
            ..wgpu_glyph::Section::default()
        };
        
        renderer.draw(crate::render::DrawCommand::Text(section.to_owned()));
    }
}

impl crate::IntoViewElement for Text {
    fn into_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}