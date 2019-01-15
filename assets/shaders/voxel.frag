#version 140

in vec3 v_normal;
in vec3 v_color;
in vec2 v_uv;

out vec4 f_color;

uniform sampler2D tex;

const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

void main() {
    // Determine the lighting
    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);

    // Apply the vertex color
    color *= v_color;

    // Get the fragment color
    f_color = vec4(color, 1.0);

    // Apply the fragment color
    f_color *= texture(tex, v_uv);
}
