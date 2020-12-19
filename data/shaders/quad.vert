#version 450
#extension GL_ARB_separate_shader_objects : enable

//Consts
layout(constant_id = 0) const float scale = 1.2f;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

//Inputs
layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_uv;

// Outputs
layout(location = 0) out vec2 v_uv;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    gl_Position = vec4(a_pos, 1.0) * ubo.model * ubo.view * ubo.proj;
    v_uv = a_uv;
}