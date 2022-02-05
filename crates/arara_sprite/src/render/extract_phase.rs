use arara_asset::{Assets, Handle};
use arara_ecs::{
    query::With,
    system::{Query, Res, ResMut},
};
use arara_render::{Color, Image, Visibility};
use arara_transform::GlobalTransform;

use crate::{
    sprite::{ExtractedSprite, Sprite},
    texture_atlas::{TextureAtlas, TextureAtlasSprite, TextureAtlasCoord},
};

#[derive(Default)]
pub struct ExtractedSprites {
    pub items: Vec<ExtractedSprite>,
}

pub(crate) fn extract_sprite_entities(
    mut extracts: ResMut<ExtractedSprites>,
    images: Res<Assets<Image>>,
    atlases: Res<Assets<TextureAtlas>>,
    query: Query<(&Handle<Image>, &GlobalTransform, &Color, &Visibility), With<Sprite>>,
    texture_atlas_query: Query<(
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &GlobalTransform,
        &Color,
        &Visibility,
    )>,
) {
    extracts.items.clear();
    for (image, transform, color, visibility) in query.iter() {
        if !visibility.active || !visibility.visible {
            continue;
        }
        if images.get(image).is_none() {
            continue;
        }
        extracts.items.push(ExtractedSprite {
            image_handle: image.clone_weak(),
            transform: transform.compute_matrix(),
            uv_coord: None,
            color: *color,
            z: transform.translation.z,
        });
    }
    for (sprite, atlas_handle, transform, color, visibility) in texture_atlas_query.iter() {
        if !visibility.active || !visibility.visible {
            continue;
        }
        if atlases.get(atlas_handle).is_none() {
            continue;
        }
        let atlas = atlases.get(atlas_handle).unwrap();
        if images.get(&atlas.texture).is_none() {
            continue;
        }
        let uv_coord = atlas.textures[sprite.index];
        let uv_coord = TextureAtlasCoord {
            point: uv_coord.point / atlas.size,
            size: uv_coord.size / atlas.size,
        };
        extracts.items.push(ExtractedSprite {
            image_handle: atlas.texture.clone_weak(),
            transform: transform.compute_matrix(),
            uv_coord: Some(uv_coord),
            color: *color,
            z: transform.translation.z,
        });
    }
}
