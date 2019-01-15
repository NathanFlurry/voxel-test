pub struct ProgramRegister {
    pub default_program: glium::Program
}

impl ProgramRegister {
    pub fn new(display: &glium::Display) -> ProgramRegister {
        ProgramRegister {
            default_program: program!(display,
                140 => {
                    vertex: include_str!("../../assets/shaders/voxel.vert"),
                    fragment: include_str!("../../assets/shaders/voxel.frag")
                }
            ).unwrap()
        }
    }
}
