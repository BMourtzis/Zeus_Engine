#version 450
#extension GL_ARB_separate_shader_objects : enable

//IN
layout(location = 0) in vec4 v_color;
layout(location = 1) in vec2 v_uv;

//UNIFORMS
layout(set = 1, binding = 0) uniform texture2D u_texture;
layout(set = 1, binding = 1) uniform sampler u_sampler;

layout(set = 2, binding = 0) uniform UBOCol {
    vec4 color;
} color_dat;

//OUT
layout(location = 0) out vec4 target0;

void main() {
    vec4 texture = texture(sampler2D(u_texture, u_sampler), v_uv);

    if (texture.w < 1) {
        discard;
    }

    target0 = texture * color_dat.color * v_color;
}
