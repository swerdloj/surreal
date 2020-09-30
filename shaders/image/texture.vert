#version 450

// In
layout(location = 0) in vec3 attr_position;
layout(location = 1) in vec2 attr_tex_coord;

// Out
layout(location = 0) out vec2 vert_tex_coord;

void main() {
    // Interpolate texture coordinates
    vert_tex_coord = attr_tex_coord;

    gl_Position = vec4(attr_position, 1.0);
}