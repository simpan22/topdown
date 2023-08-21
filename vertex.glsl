#version 330 core
in vec3 position;
in vec3 normal;
in vec2 texture_coord;

out vec3 frag_normal;
out vec3 frag_pos;
out vec2 tex_coord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    mat4 matrix = projection * view * model;
    
    vec4 world_pos = model * vec4(position, 1.0);
    vec4 screen_pos = projection * view * world_pos;

    // This is to transform the normals to world space from model space
    frag_normal = transpose(inverse(mat3(model))) * normal;
    frag_pos = vec3(model * vec4(position, 1.0));
    gl_Position = screen_pos;
    tex_coord = texture_coord;

}
