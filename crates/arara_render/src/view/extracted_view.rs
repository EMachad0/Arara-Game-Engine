use arara_camera::{Camera, Perspective};
use arara_ecs::system::{Commands, Res};
use glam::{Mat4, Vec3};

#[derive(Debug, Default)]
pub struct ExtractedView {
    pub pv_matrix: Mat4,
    pub position: Vec3,
}


pub(crate) fn extract_cameras(mut commands: Commands, camera: Res<Camera>, perspective: Res<Perspective>) {
    commands.insert_resource(ExtractedView {
        pv_matrix: perspective.calc_matrix() * camera.calc_matrix(),
        position: camera.position,
    });
}