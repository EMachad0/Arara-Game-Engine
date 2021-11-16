#version 150

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in mat4 transform;
out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;

uniform mat4 view;
uniform mat4 perspective;

void main() {
    gl_Position = perspective * view * transform * vec4(position, 1.0);
    v_normal = transpose(inverse(mat3(transform))) * normal;
    v_position = gl_Position.xyz / gl_Position.w;
    v_tex_coords = tex_coords;
}