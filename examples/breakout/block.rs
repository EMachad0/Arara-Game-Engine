use crate::{Border, GameStateResource};
use arara::prelude::*;

#[derive(Component)]
pub struct Block {
    pub cx: f32,
    pub cy: f32,
    pub width: f32,
    pub height: f32,
    pub alive: bool,
}

impl Block {
    pub fn set_dead(&mut self) {
        self.alive = false;
    }

    pub fn set_alive(&mut self) {
        self.alive = true;
    }
}

pub fn show_block(mut query: Query<(&mut Visibility, &Block)>) {
    for (mut visibility, block) in query.iter_mut() {
        visibility.active = block.alive;
    }
}

pub fn is_victory_condition(
    query: Query<&Block>,
    mut game_state: ResMut<GameStateResource>,
    mut query_border: Query<&mut Color, With<Border>>,
) {
    let mut is_victory = true;
    for block in query.iter() {
        if block.alive {
            is_victory = false;
        }
    }
    if is_victory {
        let mut border_color = query_border.single_mut();
        game_state.set_victory(&mut border_color);
    }
}
