#version 450

layout(push_constant) uniform PushConstants {
    mat4 view_proj;   // combined view * projection
} pc;

layout(location = 0) in vec3 inPos;     // Vertex.position
layout(location = 1) in float inAO;     // Vertex.ao
layout(location = 2) in vec2 inUV;      // Vertex.uv
layout(location = 3) in vec3 tint;

layout(location = 0) out vec2 fragUV;
layout(location = 1) out float fragAO;
layout(location = 2) out vec3 fragTint;

void main() {
    gl_Position = pc.view_proj * vec4(inPos, 1.0);
    fragUV = inUV;
    fragAO = inAO / 3.0; 
    fragTint = tint;
}

