#version 330 core
in vec3 position;
in vec3 normal;

out vec3 v_normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    mat4 matrix = projection * view * model;
    //v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = matrix * vec4(position, 1.0);
}
