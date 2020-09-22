use crate::state::State;
use crate::view_element::*;

use super::Widget;

use std::cell::RefMut;

// TODO: If this isn't use anywhere else later on, consider moving this type to `circle_button.rs`
pub struct TextCharacter<Msg> {
    character: char,
    font: String,
    scale: f32,
    color: Option<crate::Color>,
    _phantom_marker: std::marker::PhantomData<Msg>,
}

impl<Msg> TextCharacter<Msg> {
    pub fn font(mut self, font: &str) -> Self {
        self.font = font.to_owned();
        self
    }

    // TODO: Check if valid scale
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn color(mut self, color: crate::Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<Msg> Into<Text<Msg>> for TextCharacter<Msg> {
    fn into(self) -> Text<Msg> {
        let mut text = Text::new("__from_TextCharacter")
            .text(&self.character.to_string())
            .font(&self.font);
        
        if self.scale >= 0.0 {
            text = text.scale(self.scale);
        }

        if let Some(color) = self.color {
            text = text.color(color);
        }

        text
    }
}


#[derive(IntoViewElement)]
#[kind(Widget)]
pub struct Text<Msg> {
    id: &'static str,
    text: String,
    font: String,
    scale: f32,
    color: Option<crate::Color>,
    pub bounds: crate::bounding_rect::BoundingRect,

    message_handler: Option<Box<dyn FnMut(&mut Text<Msg>, &Msg, RefMut<State>)>>,

    section: Option<glyph_brush::OwnedSection>,

    should_resize: bool,
}

impl<Msg> Text<Msg> {
    pub fn new(id: &'static str) -> Self {
        Text {
            id,
            text: String::new(),
            font: String::from(""),
            // If negative, user did not set the scale -> use theme
            scale: -1.0,
            color: None,
            bounds: crate::bounding_rect::BoundingRect::new(),
            message_handler: None,
            section: None,
            should_resize: false,
        }
    }

    /// Descriptor for a single character. For use with `CircleButton`.
    pub fn character(character: char) -> TextCharacter<Msg> {
        TextCharacter {
            character,
            font: String::from(""),
            scale: -1.0,
            color: None,
            _phantom_marker: std::marker::PhantomData,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_owned();
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        if scale < 0.0 {
            panic!("Text scale cannot be negative (Tried setting `{}` to scale {})", self.id, scale);
        }
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

    pub fn message_handler<F: FnMut(&mut Text<Msg>, &Msg, RefMut<State>) + 'static>(mut self, handler: F) -> Self {
        self.message_handler = Some(Box::new(handler));
        self
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();

        self.should_resize = true;
    }
}

impl<Msg: EmptyMessage> Widget<Msg> for Text<Msg> where Msg: 'static{
    fn id(&self) -> &'static str {
        self.id
    }

    fn should_resize(&mut self) -> &mut bool {
        &mut self.should_resize
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

    fn handle_message(&mut self, message: &Msg, state: RefMut<State>) {
        // Bypass the borrow checker to obtain second mutable reference
        // FIXME: This exposes backend traits (Widget & IntoViewElement)
        // TODO: See if there is an alternative solution without using unsafe
        // NOTE: Regarding safety, nothing truly "unsafe" can happen here
        let this = unsafe {
            (self as *mut Text<Msg>).as_mut().unwrap()
        };
        
        if let Some(handler) = &mut self.message_handler {
            (handler)(this, message, state);
        }
    }

    fn init(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme) {       
        // Create section
        let color = if let Some(color) = &self.color {
            color
        } else {
            &theme.colors.text
        };

        if self.scale < 0.0 {
            self.scale = theme.text.scale;
        }

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
        
        // Set widget bounds
        self.bounds.width = width;
        self.bounds.height = height;

        // Store section
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