#version 150

in vec3 v_position;
in vec3 v_normal;
in vec4 v_color;
in vec2 v_tex_cords;
flat in uint v_tex_id;

out vec4 color;

uniform vec3 u_camera_pos;
uniform vec3 u_light_pos;
uniform sampler2DArray tex;

const float shineness = 32.0;
const vec3 light_color = vec3(0.3);

void main() {
    vec4 tex_color = texture(tex, vec3(v_tex_cords, float(v_tex_id))) * v_color;
    vec3 base_color = vec3(tex_color);

    vec3 normal = normalize(v_normal);
    vec3 camera_dir = normalize(u_camera_pos - v_position);
    vec3 light_dir = normalize(u_light_pos - v_position);
    vec3 half_direction = normalize(light_dir + camera_dir);
    
    float diffuse = max(dot(normal, light_dir), 0.0);
    float specular = pow(max(dot(normal, half_direction), 0.0), shineness);

    vec3 diffuse_color = base_color * diffuse;
    vec3 ambient_color = base_color * 0.1;
    vec3 specular_color = light_color * specular;

    color = vec4(ambient_color + diffuse_color + specular_color, tex_color.w); 
}