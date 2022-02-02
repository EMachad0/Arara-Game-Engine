use arara_asset::{Assets, Handle};
use arara_ecs::{
    query::With,
    system::{Query, Res, ResMut},
};
use arara_render::{Color, Image, Visibility};
use arara_transform::GlobalTransform;

use crate::sprite::{ExtractedSprite, Sprite};

#[derive(Default)]
pub struct ExtractedSprites {
    pub items: Vec<ExtractedSprite>,
}

pub(crate) fn extract_sprite_entities(
    mut extracts: ResMut<ExtractedSprites>,
    images: Res<Assets<Image>>,
    query: Query<(&Handle<Image>, &GlobalTransform, &Color, &Visibility), With<Sprite>>,
) {
    extracts.items.clear();
    for (image, global_transform, color, visibility) in query.iter() {
        if !visibility.active || !visibility.visible {
            continue;
        }
        if images.get(image).is_none() {
            continue;
        }
        extracts.items.push(ExtractedSprite {
            image_handle: image.clone_weak(),
            transform: global_transform.compute_matrix(),
            color: *color,
            z: global_transform.translation.z,
        });
    }
}
