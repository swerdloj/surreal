pub mod state;
pub mod view;
pub mod font;
pub mod application;
pub mod widget;
pub mod rectangle;
pub mod render;

use crate::widget::Widget;
use crate::view::View;

pub const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub enum EventResponse {
    /// Event will be consumed, preventing it from propogating any further.
    ///
    /// For example, a button will `Consume` left click events, 
    /// preventing other widgets from seeing that event.
    Consume,
    /// No response given. The event will propogate to other elements.
    None,
}

/// f32 RGBA color
// TODO: Consider making a Color! macro that accepts RGB, RGBA, hex, etc.
// TODO: Should these be f64, or does it not matter?
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color { r: 1., g: 1., b: 1., a: 1. };
    pub const BLACK: Color = Color { r: 0., g: 0., b: 0., a: 1. };
    pub const AUBERGINE: Color = Color { r: 0.03, g: 0.0, b: 0.02, a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {r, g, b, a}
    }
    
    pub fn from<T: Into<[f32; 4]>>(color: T) -> Self {
        let [r, g, b, a] = color.into();
        Self::new(r, g, b, a)
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color { r: self.r as f64, g: self.g as f64, b: self.b as f64, a: self.a as f64 }
    }
}

/// Element orientation
pub enum Orientation {
    Vertical,
    Horizontal,
}

pub enum ViewElement {
    Widget(Box<dyn Widget>),
    View(Box<dyn View>),
    
    // TEMP: State will be removed once procedural macro is implemented
    #[allow(non_camel_case_types)]
    TEMP_State(crate::state::State),
}

// TODO: Create derive macro for this that lets user choose element type to conver to
pub trait IntoViewElement {
    fn into_element(self) -> ViewElement;
}