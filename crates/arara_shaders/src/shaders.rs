pub struct Shaders {
    pub vertex_shader: &'static str,
    pub fragment_shader: &'static str,
}

impl Default for Shaders {
    fn default() -> Self {
        Self::from_color()
    }
}

impl Shaders {
    pub fn from_color() -> Self {
        Self {
            vertex_shader: include_str!("../../../assets/shaders/vertex_shader_src.glsl"),
            fragment_shader: include_str!("../../../assets/shaders/fragment_shader_src.glsl"),
        }
    }

    pub fn from_texture() -> Self {
        Self {
            vertex_shader: include_str!("../../../assets/shaders/vertex_shader_tex_src.glsl"),
            fragment_shader: include_str!("../../../assets/shaders/fragment_shader_tex_src.glsl"),
        }
    }
}