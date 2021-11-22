use bevy_ecs::prelude::*;

use arara_app::Plugin;
use arara_geometry::Cuboid;
use arara_transform::Transform;

use crate::{Color, SimpleMeshBundle};

pub struct CoordinateSystem {
    pub count: u32,
    pub lenght: f32,
    pub radius: f32,
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        Self {
            count: 5,
            lenght: 1.0,
            radius: 0.05,
        }
    }
}

#[derive(Default)]
pub struct CoordinateSystemPlugin;

impl Plugin for CoordinateSystemPlugin {
    fn build(&self, app_builder: &mut arara_app::AppBuilder) {
        app_builder.init_resource::<CoordinateSystem>().add_startup_system(draw_cordinate_system.system());
    }
}

fn draw_cordinate_system(mut commands: Commands, coordinate_system: Res<CoordinateSystem>) {
    let count = coordinate_system.count;
    let lenght = coordinate_system.lenght;
    let radius = coordinate_system.radius;
    for i in 0..count {
        let pos = (i as f32 + 0.5) * lenght;
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(0.9 * lenght, radius, radius)),
            transform: Transform::from_xyz(pos, 0.0, 0.0),
            color: Color::RED,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(radius, 0.9 * lenght, radius)),
            transform: Transform::from_xyz(0.0, pos, 0.0),
            color: Color::GREEN,
            ..Default::default()
        });
        commands.spawn_bundle(SimpleMeshBundle {
            mesh: Box::new(Cuboid::new(radius, radius, 0.9 * lenght)),
            transform: Transform::from_xyz(0.0, 0.0, pos),
            color: Color::BLUE,
            ..Default::default()
        });
    }
}
