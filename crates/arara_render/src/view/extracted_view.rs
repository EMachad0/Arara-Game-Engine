use arara_camera::Camera;
use arara_ecs::system::{Commands, Query};
use arara_transform::GlobalTransform;
use arara_utils::tracing::error;
use glam::{Mat4, Vec3};

#[derive(Debug, Default)]
pub struct ExtractedView {
    pub pv_matrix: Mat4,
    pub position: Vec3,
}

pub(crate) fn extract_cameras(mut commands: Commands, query: Query<(&Camera, &GlobalTransform)>) {
    let (camera, transform) = query.get_single().unwrap_or_else(|_| {
        error!("Missing Camera");
        panic!("Missing Camera");
    });
    commands.insert_resource(ExtractedView {
        pv_matrix: camera.projection * transform.view_matrix(),
        position: transform.translation,
    });
}
