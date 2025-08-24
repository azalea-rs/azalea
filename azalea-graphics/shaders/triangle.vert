
#version 450

layout(location = 0) in vec3 inPos;
layout(location = 1) in vec3 inColor;
layout(location = 2) in float inAO;

layout(push_constant) uniform PushConstants {
    mat4 mvp;
} push;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = push.mvp * vec4(inPos, 1.0);
    fragColor = inColor * inAO;
}

