use arara_app::{App, CoreStage, Plugin};
use arara_camera::Camera;
use arara_ecs::prelude::*;
use arara_transform::{GlobalTransform, Transform};
use glam::{Mat3, Quat, Vec3};

pub struct BillboardPlugin;

impl Plugin for BillboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, rotate_billboards.system());
    }
}

#[derive(Clone, Copy, Component)]
pub enum Billboard {
    /// Billboard focused on the view plane
    ViewPlane,
    /// Billboard focused on the center of the camera
    ViewPoint,
    /// Billboard focused on the view plane
    /// Doesn't rotate to match the camera pitch
    AxialViewPlane,
    /// Billboard focused on the center of the camera
    /// Doesn't rotate to match the camera pitch
    AxialViewPoint,
}

fn rotate_billboards(
    cameras: Query<(&GlobalTransform, With<Camera>)>,
    mut query: Query<(&mut Transform, &GlobalTransform, &Billboard)>,
) {
    let iterator = query.iter_mut();
    if iterator.len() == 0 {
        return;
    }

    let camera_transform = cameras.iter().last().unwrap().0;
    let camera_position = camera_transform.translation;

    let view_plane_normal = camera_transform.rotation * Vec3::Z;
    let view_plane_billboard_rotation = calc_billboard_rotation(view_plane_normal, false);
    let view_plane_axial_billboard_rotation = calc_billboard_rotation(view_plane_normal, true);

    for (mut transform, global_transform, billboard) in iterator {
        let rotation = match billboard {
            Billboard::ViewPlane => view_plane_billboard_rotation,
            Billboard::ViewPoint => {
                let normal = (camera_position - global_transform.translation).normalize();
                calc_billboard_rotation(normal, false)
            }
            Billboard::AxialViewPlane => view_plane_axial_billboard_rotation,
            Billboard::AxialViewPoint => {
                let normal = (camera_position - global_transform.translation).normalize();
                calc_billboard_rotation(normal, true)
            }
        };
        transform.rotation = rotation;
    }
}

pub fn calc_billboard_rotation(mut normal: Vec3, axial: bool) -> Quat {
    let mut up = Vec3::Y;
    let right = up.cross(normal).normalize();
    if axial {
        normal = right.cross(up).normalize();
    } else {
        up = normal.cross(right).normalize();
    }
    Quat::from_mat3(&Mat3::from_cols(right, up, normal))
}
