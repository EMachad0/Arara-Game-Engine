use arara_asset::Handle;
use bevy_ecs::prelude::*;

use crate::{Color, Image};

pub struct Translucent;

pub fn filter_translucent(mut commands: Commands, query: Query<(Entity, &Color, &Option::<Handle<Image>>)>) {
    for (entity, color, _image_handle) in query.iter() {
        if (color.a() < 1.0) {
            commands.entity(entity).insert(Translucent);
        } else {
            commands.entity(entity);
        }
    }
}