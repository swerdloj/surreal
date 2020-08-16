/*
TODO:

This module will be for everything related to rendering.
This includes quads and shaders.

The idea is to create a basic canvas-like API for rendering.
Example of what this would look like:

// Example 1
Button::render(render_target, ...) {
    render_target.rounded_rect(RoundedRect {
        at: self.rect.position,
        size: self.rect.size,
        color: Color::Red,
        border: Color::Black,
        border_width: 5,
    });
}

// Example 2
Thing::render(render_target, ...) {
    render_target.circle(Circle {
        radius: 10,
        ...
    });

    render_target.rect(Rect {
        ...
    });
}

*/

macro_rules! size_of {
    // Size of type
    ($T:ty) => {
        std::mem::size_of::<$T>()
    };
    
    // (Dynamic) Size of pointed-to value
    (var $I:ident) => {
        std::mem::size_of_val(&$I)
    };
}

pub mod quad;

pub struct Renderer {

}

impl Renderer {
    pub fn circle(&self, center: (i32, i32), radius: u32, color: crate::Color) {

    }

    pub fn rect(&self, top_left: (i32, i32), width: u32, height: u32, color: crate::Color) {

    }

    pub fn rounded_rect(&self, top_left: (i32, i32), width: u32, height: u32, roundness: u32, color: crate::Color) {

    }
}