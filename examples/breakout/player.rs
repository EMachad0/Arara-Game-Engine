use crate::{Board, GameState, GameStateResource};
use arara::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub fn move_player(
    mut query: QuerySet<(
        QueryState<(&mut Transform, &Player)>,
        QueryState<&Transform, With<Board>>,
    )>,
    game_state: Res<GameStateResource>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let board_transform = query.q1().single();
    let board_position = board_transform.translation.x;
    let board_size = board_transform.scale.x;

    let mut player_query = query.q0();
    let (mut player_transform, player) = player_query.single_mut();
    let player_size = player_transform.scale.x / 2.0;
    let player_speed = 300.0 * player.speed * time.delta_seconds();
    if let GameState::PLAYING = game_state.game_state {
        if keyboard.pressed(KeyCode::Left) {
            player_transform.translation.x -= player_speed;
        } else if keyboard.pressed(KeyCode::Right) {
            player_transform.translation.x += player_speed;
        }
    }
    player_transform.translation.x = (player_transform.translation.x).clamp(
        board_position - board_size / 2.0 + player_size,
        board_position + board_size / 2.0 - player_size,
    );
}
