#version 450

layout(push_constant) uniform PushConstants {
    vec2 screen_size;
} pc;

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_tc;
layout(location = 2) in vec4 a_srgba; // 0-1 sRGB (already normalized)

layout(location = 0) out vec4 v_rgba_in_gamma;
layout(location = 1) out vec2 v_tc;

void main() {
    gl_Position = vec4(
        2.0 * a_pos.x / pc.screen_size.x - 1.0,
        2.0 * a_pos.y / pc.screen_size.y - 1.0,
        0.0,
        1.0
    );
    v_rgba_in_gamma = a_srgba;
    v_tc = a_tc;
} 