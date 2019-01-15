#version 140

uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform mat4 projection_matrix;

in vec3 position;
in vec3 normal;
in vec3 color;
in vec2 uv;

// TODO: Add uv

out vec3 v_position;
out vec3 v_normal;
out vec3 v_color;
out vec2 v_uv;

void main() {
    v_position = position;
    v_normal = normal;
    v_color = color;
    v_uv = uv;
    gl_Position = projection_matrix * view_matrix * model_matrix * vec4(v_position, 1.0);
}
