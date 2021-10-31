#![allow(dead_code)]

mod transform;
mod transform_builder;

pub mod consts {
    use cgmath::*;
    
    pub const X_AXIS : Vector3::<f32> = Vector3::<f32>::new(1.0, 0.0, 0.0);
    pub const Y_AXIS : Vector3::<f32> = Vector3::<f32>::new(0.0, 1.0, 0.0);
    pub const Z_AXIS : Vector3::<f32> = Vector3::<f32>::new(0.0, 0.0, 1.0);
}

pub use transform_builder::TransformBuilder;