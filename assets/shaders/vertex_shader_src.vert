#version 150

uniform camera {
    mat4 u_pv_matrix;
};

in vec3 i_position;
in vec3 i_normal;
in vec4 i_color;
in vec2 i_tex_cords;
in uint i_tex_id;

out vec3 v_position;
out vec3 v_normal;
out vec4 v_color;
out vec2 v_tex_cords;
flat out uint v_tex_id;

void main() {
    gl_Position = u_pv_matrix * vec4(i_position, 1.0);
    // v_normal = transpose(inverse(mat3(u_transform))) * normal;
    v_normal = i_normal;
    v_position = i_position;
    v_color = i_color;
    v_tex_cords = i_tex_cords;
    v_tex_id = i_tex_id;
}