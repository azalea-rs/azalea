#version 450

layout(push_constant) uniform PushConstants {
    mat4 view_proj;   // combined view * projection
} pc;

layout(location = 0) in vec3 inPos;     // Vertex.position
layout(location = 1) in float inAO;     // Vertex.ao
layout(location = 2) in uint inTexIdx;  // Vertex.tex_idx
layout(location = 3) in vec2 inUV;      // Vertex.uv

layout(location = 0) out vec2 fragUV;
layout(location = 1) flat out uint fragTexIdx;
layout(location = 2) out float fragAO;

void main() {
    gl_Position = pc.view_proj * vec4(inPos, 1.0);
    fragUV = inUV;
    fragTexIdx = inTexIdx;
    fragAO = inAO / 3.0;  // normalize AO (0..3) → (0..1)
}

