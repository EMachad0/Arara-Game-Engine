use arara_ecs::component::Component;
use glam::Mat4;

#[derive(Component, Debug, Default)]
pub struct Camera {
    pub projection: Mat4,
}
