use arara_asset::{Assets, Handle};
use arara_ecs::prelude::*;
use arara_window::Window;
use glam::vec4;
use glium::texture::{RawImage2d, SrgbTexture2d};

use crate::{render_phase::RenderPhase, ExtractedCPE, ExtractedView, Image, Opaque, Transparent, TextureBuffer};

pub fn prepare_split_render_phase(
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

pub fn prepare_bindless_textures(
    window: NonSend<Window>,
    images: Res<Assets<Image>>,
    mut texture_buffer: NonSendMut<TextureBuffer>,
    query: Query<(&Handle<Image>, With<ExtractedCPE>)>,
) {
    let display = window.display();
    for (image_handle, _) in query.iter() {
        if texture_buffer.contains(image_handle) {
            continue;
        }
        let image = images.get(image_handle).unwrap();
        let raw_image = RawImage2d::from_raw_rgba_reversed(&image.data, image.dimensions);
        let texture = SrgbTexture2d::new(display, raw_image).unwrap();
        texture_buffer.insert_or_update(image_handle.clone_weak(), texture);
    }
}
