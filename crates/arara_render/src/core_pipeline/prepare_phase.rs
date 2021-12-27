use arara_asset::{Assets, Handle};
use arara_ecs::prelude::*;
use arara_transform::GlobalTransform;
use glam::{vec4, Mat4};

use crate::{
    render_phase::RenderPhase, Color, ExtractedView, Image, Mesh, Opaque, Transparent, Visibility,
};

pub fn prepare_core_pass(
    mut opaques: ResMut<RenderPhase<Opaque>>,
    mut transparents: ResMut<RenderPhase<Transparent>>,
    view: Res<ExtractedView>,
    meshes: Res<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    query: Query<(
        Entity,
        &Handle<Mesh>,
        &GlobalTransform,
        &Color,
        &Handle<Image>,
        &Visibility,
    )>,
) {
    let pv_matrix = view.pv_matrix.to_cols_array_2d();

    for (entity, mesh, global_transform, color, image_handle, visibility) in query.iter() {
        if !visibility.active || !visibility.visible {
            continue;
        }

        let mut transparent = if *image_handle == Handle::<Image>::default() {
            false
        } else {
            match images.get(image_handle) {
                Some(image) => image.translucent,
                None => continue,
            }
        };

        transparent |= color.a() < 1.0;

        if meshes.get(mesh).is_none() {
            continue;
        }

        let transform = global_transform.compute_matrix();
        let position = Mat4::from_cols_array_2d(&pv_matrix) * transform * vec4(0., 0., 0., 1.);
        let distance = -position.z.abs();

        if transparent {
            transparents.add(Transparent { distance, entity });
        } else {
            opaques.add(Opaque { distance, entity });
        }
    }
}
