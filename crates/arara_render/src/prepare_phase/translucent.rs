use arara_asset::Handle;
use bevy_ecs::prelude::Query;

use crate::{Color, Image};

pub fn filter_translucent(query: Query<(&Color, &Option::<Handle<Image>>)>) {

}