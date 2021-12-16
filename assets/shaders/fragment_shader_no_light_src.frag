#version 150

in vec3 v_position;
in vec3 v_normal;
in vec4 v_color;
in vec2 v_tex_cords;
flat in uint v_tex_id;

out vec4 color;

uniform sampler2DArray tex;

void main() {
    vec4 tex_color = texture(tex, vec3(v_tex_cords, float(v_tex_id))) * v_color;
    color = tex_color; 
}