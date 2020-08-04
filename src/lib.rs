pub mod state;
pub mod view;
#[macro_use]
pub mod font;
pub mod application;
pub mod widget;
pub mod rectangle;

use crate::widget::Widget;
use crate::view::View;

/// f32 RGBA color
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {r, g, b, a}
    }
    
    pub fn from<T: Into<[f32; 4]>>(color: T) -> Self {
        let [r, g, b, a] = color.into();
        Self::new(r, g, b, a)
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
    fn as_element(self) -> ViewElement;
}