pub struct ProgramRegister {
    pub default_program: glium::Program
}

impl ProgramRegister {
    pub fn new(display: &glium::Display) -> ProgramRegister {
        ProgramRegister {
            default_program: program!(display,
                140 => {
                    vertex: "
                        #version 140

                        uniform mat4 persp_matrix;
                        uniform mat4 view_matrix;

                        in vec3 position;
                        in vec3 normal;
                        in vec3 color;

                        // TODO: Add uv

                        out vec3 v_position;
                        out vec3 v_normal;
                        out vec3 v_color;

                        void main() {
                            v_position = position;
                            v_normal = normal;
                            v_color = color;
                            gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                        }
                    ",

                    fragment: "
                        #version 140

                        in vec3 v_normal;
                        in vec3 v_color;

                        out vec4 f_color;

                        const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                        void main() {
                            float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                            vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                            color *= v_color;
                            f_color = vec4(color, 1.0);
                        }
                    "
                }
            ).unwrap()
        }
    }
}
