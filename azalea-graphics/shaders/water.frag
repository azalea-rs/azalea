#version 450

layout(set = 0, binding = 0) uniform sampler2D blocks_atlas;

layout(location = 0) in vec2 fragUV;
layout(location = 1) in float fragAO;
layout(location = 2) in vec3 fragTint;

layout(location = 0) out vec4 outColor;

void main() {
    vec4 texColor = texture(blocks_atlas, fragUV);
    
    // Water transparency - make water semi-transparent
    float alpha = texColor.a;

    
    outColor = vec4(texColor.rgb * fragTint * fragAO, alpha);
}
