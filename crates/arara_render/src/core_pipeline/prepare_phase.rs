use arara_asset::Assets;
use arara_ecs::prelude::*;
use glam::vec4;

use crate::{
    render_phase::RenderPhase, ExtractedCPE, ExtractedView, Image, Opaque,
    Transparent,
};

pub fn prepare_core_pass(
    mut opaques: ResMut<RenderPhase<Opaque>>,
    mut transparents: ResMut<RenderPhase<Transparent>>,
    view: Res<ExtractedView>,
    images: Res<Assets<Image>>,
    query: Query<(Entity, &ExtractedCPE)>,
) {
    for (
        entity,
        ExtractedCPE {
            mesh: _,
            image,
            transform,
            color,
        },
    ) in query.iter()
    {
        let transparent = images.get(image).unwrap().translucent || color.a() < 1.0;

        let position = view.pv_matrix * *transform * vec4(0., 0., 0., 1.);
        let distance = -position.z.abs();

        if transparent {
            transparents.add(Transparent { distance, entity });
        } else {
            opaques.add(Opaque { distance, entity });
        }
    }
}
