#version 450

#define RECTANGLE 0
#define ROUNDED_RECT 1
#define CIRCLE 2

layout(set = 0, binding = 0)
uniform Uniforms {
    vec4 color;
    vec2 window_dimensions;
    vec2 primitive_center;
    uint type;
    float circle_radius;
    float primitive_width;
    float primitive_height;
    float rounded_rect_roundness;
};

layout(location = 0) out vec4 out_color;

// SDFs modified from iq
// https://www.iquilezles.org/www/articles/distfunctions2d/distfunctions2d.htm

// TODO: Do not use uv-coords -> update everything to remove them
// Pixel-coords work perfectly fine and are much easier to use

// TODO: Alpha-blending for alread-transparent colors

float sd_circle(vec2 point, vec2 center, float radius) {
    return length(point - center) - radius;
}

float sd_rounded_rect(vec2 point, vec2 center, float half_width, float half_height, float roundness) {
    return length(max(abs(point - center) - vec2(half_width, half_height) + roundness, 0.0)) - roundness;
}

float alpha_from_dist(float dist) {
    return min(1.0, 1.0 - dist);
}

void main() {
    if (type == RECTANGLE) {
        out_color = color;
        return;
    } 

    else if (type == ROUNDED_RECT) {
        // TODO: This
        float dist = sd_rounded_rect(gl_FragCoord.xy, primitive_center, primitive_width, primitive_height, rounded_rect_roundness);
        out_color = vec4(color.rgb, alpha_from_dist(dist));
        return;
    } 
    
    else if (type == CIRCLE) {
        float dist = sd_circle(gl_FragCoord.xy, primitive_center, circle_radius);
        // FIXME: Without this, the top and left sides look aliased (or cut off?)
        // This seems to be because I used a rect to represent the circle (rounding issue)
        // Once I actually implement circles, this needs to be double-checked
        dist += 0.5;
        out_color = vec4(color.rgb, alpha_from_dist(dist));
        return;
    }
}