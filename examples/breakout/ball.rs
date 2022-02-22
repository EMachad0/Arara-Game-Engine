use crate::{Block, Board, Border, GameState, GameStateResource, Player};
use arara::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub speed: f32,
    pub velocity: Vec2,
}

pub fn move_ball(
    mut query_ball: Query<(&mut Transform, &Ball)>,
    game_state: Res<GameStateResource>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let GameState::PLAYING = game_state.game_state {
        let (mut transform, ball) = query_ball.single_mut();
        let mut speedy = 300.0 * ball.speed * time.delta_seconds() * ball.velocity;
        if keyboard.pressed(KeyCode::Space) {
            speedy *= 2.0;
        }
        transform.translation += Vec3::from((speedy, 0.0));
    }
}

pub fn calculate_collision(
    mut query: QuerySet<(
        QueryState<(&Transform, &mut Ball)>,
        QueryState<&Transform, With<Player>>,
        QueryState<&Transform, With<Board>>,
    )>,
    mut query_block: Query<&mut Block>,
    mut query_border: Query<&mut Color, With<Border>>,
    mut game_state: ResMut<GameStateResource>,
) {
    // Get board position
    let board_transform = query.q2().single();
    let board_x = board_transform.translation.x;
    let board_y = board_transform.translation.y;
    let board_width = board_transform.scale.x / 2.0;
    let board_height = board_transform.scale.y / 2.0;

    // Get player position
    let player_transform = query.q1().single();
    let player_x = player_transform.translation.x;
    let player_y = player_transform.translation.y;
    let player_width = player_transform.scale.x / 2.0;
    let player_height = player_transform.scale.y / 2.0;

    // Get ball position
    let mut ball_query = query.q0();
    let (ball_transform, mut ball) = ball_query.single_mut();
    let ball_x = ball_transform.translation.x;
    let ball_y = ball_transform.translation.y;
    let ball_size = ball_transform.scale.x / 2.0;

    // Left wall
    if ball_x - ball_size < board_x - board_width {
        ball.velocity[0] = -ball.velocity[0];
    }

    // Right wall
    if ball_x + ball_size > board_x + board_width {
        ball.velocity[0] = -ball.velocity[0];
    }

    // Top wall
    if ball_y + ball_size > board_y + board_height {
        ball.velocity[1] = -ball.velocity[1];
    }

    // Bottom wall
    if ball_y - ball_size < board_y - board_height {
        ball.velocity[1] = -ball.velocity[1];
        let mut border_color = query_border.single_mut();
        game_state.set_defeat(&mut border_color);
    }

    // Top player
    if (ball_x + ball_size > player_x - player_width)
        && (ball_x - ball_size < player_x + player_width)
        && (ball_y - ball_size <= player_y + player_height)
        && (ball_y + ball_size > player_y - player_height)
    {
        let mut aux = 1.8 / (player_width * 2.0 + ball_size);
        let dist = ball_x - (player_x - player_width) + ball_size;
        let new_velocity_0 = -0.9 + aux * dist;
        ball.velocity[0] = new_velocity_0;
        if new_velocity_0 < 0.0 {
            aux = -new_velocity_0;
        } else {
            aux = new_velocity_0;
        }
        ball.velocity[1] = 1.0 - aux;
        ball.speed += 0.01;
    }

    for mut block in query_block.iter_mut() {
        if !block.alive {
            continue;
        }
        // Left block

        if (ball_x + ball_size <= block.cx - block.width / 2.0)
            && (ball_x - ball_size > block.cx - block.width / 2.0)
            && (ball_y + ball_size > block.cy - block.height / 2.0)
            && (ball_y - ball_size < block.cy + block.height / 2.0)
        {
            ball.velocity[0] = -ball.velocity[0];
            block.set_dead();
        }
        // Right block
        if (ball_x + ball_size > block.cx + block.width / 2.0)
            && (ball_x - ball_size <= block.cx + block.width / 2.0)
            && (ball_y + ball_size > block.cy - block.height / 2.0)
            && (ball_y - ball_size < block.cy + block.height / 2.0)
        {
            ball.velocity[0] = -ball.velocity[0];
            block.set_dead();
        }

        // Top block
        if (ball_x + ball_size > block.cx - block.width / 2.0)
            && (ball_x - ball_size < block.cx + block.width / 2.0)
            && (ball_y - ball_size <= block.cy + block.height / 2.0)
            && (ball_y + ball_size > block.cy + block.height / 2.0)
        {
            ball.velocity[1] = -ball.velocity[1];
            block.set_dead();
        }

        // Bottom block
        if (ball_x + ball_size > block.cx - block.width / 2.0)
            && (ball_x - ball_size < block.cx + block.width / 2.0)
            && (ball_y - ball_size < block.cy - block.height / 2.0)
            && (ball_y + ball_size >= block.cy - block.height / 2.0)
        {
            ball.velocity[1] = -ball.velocity[1];
            block.set_dead();
        }
    }
}
