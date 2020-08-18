#version 450

layout(set = 0, binding = 0)
uniform Uniforms {
    vec4 color;
};

layout(location = 0) out vec4 out_color;

// SDFs modified from iq
// https://www.iquilezles.org/www/articles/distfunctions2d/distfunctions2d.htm

float sd_circle(vec2 center, float radius) {
    return length(gl_FragCoord.xy - center) - radius;
}

float sd_rounded_rect(vec2 center, float width, float height, float roundness) {
    vec2 q = abs(gl_FragCoord.xy - center) - vec2(width, height) + roundness;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - roundness;
}

void main() {
    out_color = color;
}