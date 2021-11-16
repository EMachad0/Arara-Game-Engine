#version 150

in vec3 position;
in vec3 normal;

out vec3 v_position;
out vec3 v_normal;

uniform mat4 u_pv_matrix;
uniform mat4 u_transform;

void main() {
    gl_Position = u_pv_matrix * u_transform * vec4(position, 1.0);
    v_normal = transpose(inverse(mat3(u_transform))) * normal;
    v_position = gl_Position.xyz / gl_Position.w;
}