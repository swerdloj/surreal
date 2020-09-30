#version 450

// Uniform
layout(set = 0, binding = 0) uniform texture2D image_texture;
layout(set = 0, binding = 1) uniform sampler texture_sampler;

// In
layout(location = 0) in vec2 vert_tex_coord;

// Out
layout(location = 0) out vec4 out_color;

void main() {
    vec4 texture_color = texture(
        sampler2D(image_texture, texture_sampler), 
        vert_tex_coord
    );

    out_color = texture_color;
}