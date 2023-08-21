#version 330 core
in vec3 frag_normal;
in vec3 frag_pos;
in vec2 tex_coord;

out vec4 color;

uniform vec3 light_pos;
uniform vec3 light_color;
uniform mat4 view;
uniform vec3 object_color;
uniform sampler2D texture_sampler;

void main() {
    // Colors
    vec3 object_color = texture(texture_sampler, tex_coord).rgb;;
    vec3 diffuse_color = light_color;
    vec3 specular_color = light_color;
    vec3 ambient_color = vec3(1.0, 1.0, 1.0);

    // Parameters
    float ambient_brightness = 0.1;


    // Parameters needed for light calculation
    vec3 view_pos = -vec3(view[3][0], view[3][1], view[3][2]);
    vec3 light_dir = normalize(light_pos - frag_pos);
    vec3 view_dir = normalize(view_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, normalize(frag_normal));

    // Light calculations
    float diffuse = clamp(dot(normalize(frag_normal), light_dir), 0, 1);

    vec3 specular = pow(max(dot(view_dir, reflect_dir), 0.0), 32) * specular_color;


    vec3 result = (diffuse +/* specular +*/ ambient_color * ambient_brightness) * object_color;
    color = vec4(result, 1.0);
}
