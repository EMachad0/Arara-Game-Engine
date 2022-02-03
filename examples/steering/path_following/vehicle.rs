use arara::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Speedy {
    pub speed: Vec2,
    pub max_speed: f32,
    pub acceleration: Vec2,
    pub max_force: f32,
}

impl Speedy {
    pub fn add_force(&mut self, force: &Vec2) {
        self.acceleration += *force;
    }
}

pub fn move_speedy(mut query: Query<(&mut Transform, &mut Speedy)>) {
    for (mut transform, mut speedy) in query.iter_mut() {
        let acceleration = speedy.acceleration.clamp_length_max(speedy.max_force);
        speedy.speed += acceleration;
        speedy.speed = speedy.speed.clamp_length_max(speedy.max_speed);
        transform.translation += Vec3::from((speedy.speed, 0.0));
        transform.rotation = Quat::from_rotation_arc_2d(Vec2::X, speedy.speed.normalize());
        speedy.acceleration = Vec2::ZERO;
    }
}
