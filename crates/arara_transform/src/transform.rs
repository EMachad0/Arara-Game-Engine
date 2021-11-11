#[derive(Copy, Clone)]
pub struct Transform {
    transform: [[f32; 4]; 4],
}

glium::implement_vertex!(Transform, transform);

impl Transform {
    pub fn new(transform: [[f32; 4]; 4]) -> Self {
        Self {
            transform,
        }
    }
}