#version 450

layout(set = 0, binding = 0) uniform sampler2D blocks_atlas; // big texture array

layout(location = 0) in vec2 fragUV;
layout(location = 1) in float fragAO;

layout(location = 0) out vec4 outColor;

void main() {
    vec4 texColor = texture(blocks_atlas, fragUV);
    if (texColor.w < 0.5) {
      discard;
    }

    outColor = vec4(texColor.rgb * fragAO, texColor.a);
}
