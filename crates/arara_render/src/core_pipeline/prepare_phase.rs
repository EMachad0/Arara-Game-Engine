use arara_asset::{Handle, Assets};
use arara_camera::FlyCamera;
use arara_transform::GlobalTransform;
use bevy_ecs::prelude::*;
use glam::{Mat4, vec4};

use crate::{Mesh, Color, Image, Visibility, Opaque, Transparent, render_phase::RenderPhase};

pub fn prepare_core_pass(
    mut opaques: ResMut<RenderPhase::<Opaque>>,
    mut transparents: ResMut<RenderPhase::<Transparent>>,
    mut fly_camera: ResMut<FlyCamera>,
    meshes: Res<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    query: Query<(Entity, &Handle<Mesh>, &GlobalTransform, &Color, &Option::<Handle<Image>>, &Visibility)>,
) {
    let pv_matrix = fly_camera.calc_matrix();

    for (entity, mesh, global_transform, color, image_handle, visibility) in query.iter() {
        if !visibility.active || !visibility.visible {
            continue;
        }

        let mut transparent = match image_handle {
            Some(handle) => match images.get(handle) {
                Some(image) => image.translucent,
                None => continue,
            }
            None => false,
        };

        transparent |= color.a() < 1.0;

        if meshes.get(mesh).is_none() {
            continue;
        }

        let transform = global_transform.compute_matrix();
        let position = Mat4::from_cols_array_2d(&pv_matrix) * transform * vec4(0., 0., 0., 1.);
        let distance = -position.z.abs();

        if transparent {
            transparents.add(Transparent {
                distance,
                entity,
            });
        } else {
            opaques.add(Opaque {
                distance,
                entity,
            });
        }
    }
}