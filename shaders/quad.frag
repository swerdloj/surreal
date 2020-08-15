#version 450

layout(set = 0, binding = 0)
uniform Uniforms {
    vec3 test;
};

layout(location = 0) out vec4 out_color;

void main() {
    out_color = vec4(test, 1.0);
}