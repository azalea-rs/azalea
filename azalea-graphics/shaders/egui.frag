#version 450

layout(set = 0, binding = 0) uniform sampler2D u_sampler;

layout(location = 0) in vec4 v_rgba_in_gamma;
layout(location = 1) in vec2 v_tc;

layout(location = 0) out vec4 outColor;

void main() {
    vec4 texture_in_gamma = texture(u_sampler, v_tc);
    
    vec4 frag_color_gamma = v_rgba_in_gamma * texture_in_gamma;
    
    outColor = frag_color_gamma;
} 