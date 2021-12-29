#version 460
#extension GL_ARB_bindless_texture : require

in vec3 v_position;
in vec3 v_normal;
in vec4 v_color;
in vec2 v_tex_cords;
flat in uint v_tex_id;

out vec4 color;

uniform samplers {
    sampler2D tex[5];
};

void main() {
    vec4 tex_color = texture(tex[v_tex_id], v_tex_cords) * v_color;
    color = tex_color; 
}