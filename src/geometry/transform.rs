
use cgmath::*;

pub mod consts {
    use cgmath::*;
    
    pub const X_AXIS : Vector3::<f32> = Vector3::<f32>::new(1.0, 0.0, 0.0);
    pub const Y_AXIS : Vector3::<f32> = Vector3::<f32>::new(0.0, 1.0, 0.0);
    pub const Z_AXIS : Vector3::<f32> = Vector3::<f32>::new(0.0, 0.0, 1.0);
}

pub struct Transform {
    matrix: Matrix4::<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            matrix: Matrix4::<f32>::identity(),
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.matrix = Matrix4::from_scale(scale) * self.matrix;
        self
    }

    pub fn non_uniform_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.matrix = Matrix4::from_nonuniform_scale(x, y, z) * self.matrix;
        self
    }

    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.matrix = Matrix4::from_translation([x, y, z].into()) * self.matrix;
        self
    }

    pub fn rotate<A: Into<Rad<f32>>>(mut self, axis: Vector3::<f32>, angle :A) -> Self {
        self.matrix = Matrix4::from_axis_angle(axis, angle) * self.matrix;
        self
    }

    pub fn build(self) -> [[f32; 4]; 4] {
        self.matrix.into()
    }
}