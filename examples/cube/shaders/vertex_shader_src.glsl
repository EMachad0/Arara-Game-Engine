#version 150

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;

uniform mat4 matrix;

void main() {
    gl_Position = perspective * view * model * vec4(position, 1.0);
    v_normal = transpose(inverse(mat3(model))) * normal;
    v_position = gl_Position.xyz / gl_Position.w;
    v_tex_coords = tex_coords;
}