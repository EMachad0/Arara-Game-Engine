use cgmath::*;

use super::transform::Transform;

pub struct TransformBuilder {
    matrix: Matrix4::<f32>,
}

impl Default for TransformBuilder {
    fn default() -> Self {
        Self {
            matrix: Matrix4::<f32>::identity(),
        }
    }
}

impl TransformBuilder {
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

    pub fn build(self) -> Transform {
        Transform::new(self.matrix.into())
    }
}