#version 450
#extension GL_EXT_nonuniform_qualifier : require

layout(set = 0, binding = 0) uniform sampler2D textures[]; // big texture array

layout(location = 0) in vec2 fragUV;
layout(location = 1) flat in uint fragTexIdx;
layout(location = 2) in float fragAO;

layout(location = 0) out vec4 outColor;

void main() {
    vec4 texColor = texture(textures[fragTexIdx], fragUV);
    if (texColor.w < 0.5) {
      discard;
    }

    outColor = vec4(texColor.rgb * fragAO, texColor.a);
}
