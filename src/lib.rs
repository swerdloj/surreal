// TODO: Go through and decide what needs to be pub, pub(crate), & private
pub mod state;
pub mod view;
pub mod application;
pub mod widget;
pub mod bounding_rect;
pub mod render;
pub mod timing;
pub mod style;

use crate::widget::Widget;
use crate::view::View;

pub const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;


// Re-exports
// TODO: Identify the important items to place here
pub use render::font::IncludedFonts;


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
// TODO: Consider making a `Color!` macro that accepts RGB, RGBA, hex, etc.
// TODO: Should these be f64, or does it not matter?
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const AUBERGINE:    Color = Color { r: 0.03,  g: 0.0,  b: 0.02,  a: 1.0 };
    pub const ALMOST_WHITE: Color = Color { r: 0.9,   g: 0.9,  b: 0.9,   a: 1.0 };
    pub const BLACK:        Color = Color { r: 0.0,   g: 0.0,  b: 0.0,   a: 1.0 };
    pub const CLEAR:        Color = Color { r: 0.0,   g: 0.0,  b: 0.0,   a: 0.0 };
    pub const DARK_GRAY:    Color = Color { r: 0.01,  g: 0.01, b: 0.01,  a: 1.0 };
    pub const WHITE:        Color = Color { r: 1.0,   g: 1.0,  b: 1.0,   a: 1.0 };

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

/// Element alignment
pub enum Alignment {
    Left,
    Right,
    Center,
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