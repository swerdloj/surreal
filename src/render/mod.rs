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