use arara::prelude::*;
use glam::Vec2;

#[derive(Component, Debug)]
pub struct Path {
    pub points: Vec<Vec2>,
    pub radius: f32,
}

pub fn draw_paths(
    mut commands: Commands,
    query: Query<(Entity, &Path)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (entity, path) in query.iter() {
        commands
            .entity(entity)
            .insert_bundle(TransformBundle::default())
            .with_children(|children| {
                for i in 1..path.points.len() {
                    let p1 = path.points[i - 1];
                    let p2 = path.points[i];
                    let dir = p2 - p1;
                    children.spawn_bundle(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::from((p1 + dir / 2.0, -500.0)),
                            rotation: Quat::from_rotation_z(-dir.angle_between(Vec2::X)),
                            scale: vec3(dir.length(), path.radius, 1.0),
                            ..Default::default()
                        },
                        color: Color::rgb(0.2, 0.2, 0.2),
                        ..Default::default()
                    });
                    children.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Circle::new(16, path.radius / 2.0))),
                        transform: Transform::from_translation(Vec3::from((p1, -500.0))),
                        color: Color::rgb(0.2, 0.2, 0.2),
                        ..Default::default()
                    });
                }
            });
    }
}
