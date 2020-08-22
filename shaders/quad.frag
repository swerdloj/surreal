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

float sd_circle(vec2 uv, vec2 center, float radius) {
    return length(uv - center) - radius;
}

float sd_rounded_rect(vec2 uv, vec2 center, float half_width, float half_height, float roundness) {
    return length(max(abs(uv - center) - vec2(half_width, half_height) + roundness, 0.0)) - roundness;
}

void main() {
    if (type == RECTANGLE) {
        out_color = color;
        return;
    } 
    
    vec2 uv = (gl_FragCoord.xy / window_dimensions - 0.5) * 2.;
    uv.x *= window_dimensions.x / window_dimensions.y;


    if (type == ROUNDED_RECT) {
        float dist = sd_rounded_rect(uv, primitive_center, primitive_width, primitive_height, rounded_rect_roundness);
        if (dist <= 0) {
            out_color = color;
        } else {
            // out_color = vec4(0);
            discard;
        }
        return;
    } else if (type == CIRCLE) {
        float dist = sd_circle(uv, primitive_center, circle_radius);
        return;
    }
}