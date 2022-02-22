mod ball;
mod block;
mod player;

use arara::prelude::*;

use crate::{ball::*, block::*, player::*};

//|

fn main() {
    App::new()
        .init_resource::<WindowProps>()
        .insert_resource(WindowProps {
            width: 800,
            height: 600,
            resizable: false,
            cursor_visible: false,
            vsync: true,
            title: "Breakout".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(AssetCountDiagnosticsPlugin::<Image>::default())
        .add_plugin(LogDiagnosticPlugin::default())
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .init_resource::<GameStateResource>()
        .add_startup_system(add_camera)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, reset)
        .add_system(show_block)
        .add_system(move_player)
        .add_system(move_ball)
        .add_system_to_stage(CoreStage::PostUpdate, calculate_collision)
        .add_system_to_stage(CoreStage::PostUpdate, is_victory_condition)
        .run()
}

#[derive(Component)]
pub struct Board;

#[derive(Component)]
pub struct Border;

enum GameState {
    DEFEAT,
    PLAYING,
    VICTORY,
}

pub struct GameStateResource {
    game_state: GameState,
    pub timer: Option<Timer>,
}

impl Default for GameStateResource {
    fn default() -> Self {
        Self {
            game_state: GameState::PLAYING,
            timer: None,
        }
    }
}

impl GameStateResource {
    pub fn set_victory(&mut self, border_color: &mut Color) {
        if let GameState::VICTORY = self.game_state {
            return;
        }
        *border_color = Color::GREEN;
        self.game_state = GameState::VICTORY;
        self.timer = Some(Timer::from_seconds(1.0, false));
    }

    pub fn set_playing(&mut self, border_color: &mut Color) {
        if let GameState::PLAYING = self.game_state {
            return;
        }
        *border_color = Color::ORANGE;
        self.game_state = GameState::PLAYING;
        self.timer = None;
    }

    pub fn set_defeat(&mut self, border_color: &mut Color) {
        if let GameState::DEFEAT = self.game_state {
            return;
        }
        *border_color = Color::RED;
        self.game_state = GameState::DEFEAT;
        self.timer = Some(Timer::from_seconds(1.0, false));
    }
}

fn reset(
    mut query: QuerySet<(
        QueryState<(&mut Transform, &mut Ball)>,
        QueryState<&mut Transform, With<Player>>,
    )>,
    mut query_blocks: Query<&mut Block>,
    mut query_border: Query<&mut Color, With<Border>>,
    mut game_state: ResMut<GameStateResource>,
    time: Res<Time>,
) {
    if let GameState::PLAYING = game_state.game_state {
        return;
    }

    let timer = game_state.timer.as_mut().unwrap();
    timer.tick(time.delta());
    if timer.just_finished() {
        let mut ball_query = query.q0();
        let (mut ball_transform, mut ball) = ball_query.single_mut();
        ball_transform.translation.x = 0.0;
        ball_transform.translation.y = -255.5;
        let vx: f32 = rand::random::<f32>() * 1.8 - 0.9;
        let mut aux: f32 = vx;
        if aux < 0.0 {
            aux = -aux;
        }
        let vy = 1.0 - aux;
        ball.velocity = vec2(vx, vy);

        let mut player_query = query.q1();
        let mut player_transform = player_query.single_mut();
        player_transform.translation.x = 0.0;

        for mut block in query_blocks.iter_mut() {
            block.set_alive();
        }
        let mut border_color = query_border.single_mut();
        game_state.set_playing(&mut border_color);
    }
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>, _window: NonSend<Window>) {
    commands
        .spawn_bundle(TransformBundle::new(Transform::from_xyz(400.0, 300.0, 0.0)))
        .with_children(|parent| {
            // Border
            parent
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_scale(vec3(800.0, 600.0, 1.0)),
                    color: Color::ORANGE,
                    ..Default::default()
                })
                .insert(Border);
            // Board
            parent
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_scale(vec3(790.0, 590.0, 1.0)),
                    color: Color::DARK_GRAY,
                    ..Default::default()
                })
                .insert(Board);
            // Player
            parent
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: vec3(0.0, -267.5, 0.0),
                        scale: vec3(80.0, 10.0, 1.0),
                        ..Default::default()
                    },
                    color: Color::ORANGE_RED,
                    ..Default::default()
                })
                .insert(Player { speed: 0.7 });
            // Ball
            let vx: f32 = rand::random::<f32>() * 1.8 - 0.9;
            let mut aux: f32 = vx;
            if aux < 0.0 {
                aux = -aux;
            }
            let vy = 1.0 - aux;
            parent
                .spawn_bundle(SpriteBundle {
                    transform: Transform {
                        translation: vec3(0.0, -255.5, 0.0),
                        scale: vec3(10.0, 10.0, 1.0),
                        ..Default::default()
                    },
                    color: Color::BLACK,
                    ..Default::default()
                })
                .insert(Ball {
                    speed: 1.0,
                    velocity: vec2(vx, vy),
                });
            // Blocks
            for xx in 2..5 {
                for yy in 0..10 {
                    let x = xx as f32;
                    let y = yy as f32;
                    let cx = -355.0 + 15.0 * (y + 1.0) + 60.0 * y;
                    let cy = 283.0 - 5.0 * (x + 1.0) - 15.0 * x;
                    let width = 60.0;
                    let height = 14.0;
                    let alive = true;
                    parent
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: vec3(cx, cy, 1.0),
                                scale: vec3(width, height, 1.0),
                                ..Default::default()
                            },
                            color: Color::rgb(rand::random(), rand::random(), rand::random()),
                            ..Default::default()
                        })
                        .insert(Block {
                            cx,
                            cy,
                            width,
                            height,
                            alive,
                        });
                }
            }
        });
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::default());
}
