pub struct Shaders {
    pub vertex_shader: &'static str,
    pub fragment_shader: &'static str,
}

impl Default for Shaders {
    fn default() -> Self {
        Self {
            vertex_shader: include_str!("../../../assets/shaders/vertex_shader_src.glsl"),
            fragment_shader: include_str!("../../../assets/shaders/fragment_shader_src.glsl"),
        }
    }
}