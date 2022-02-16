use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        // .add_plugin(CoordinateSystemPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(1),
        })
        .add_startup_system(add_shapes)
        .add_startup_system(add_camera)
        .insert_resource(BPLight {
            position: vec3(-2.0, 5.0, 3.0),
        })
        .run()
}

#[derive(Component)]
struct Robot;
#[derive(Component)]
struct LeftLeg;
#[derive(Component)]
struct RightLeg;
#[derive(Component)]
struct LowerLeftLeg;
#[derive(Component)]
struct LowerRightLeg;
#[derive(Component)]
struct UpperLeftLeg;
#[derive(Component)]
struct UpperRightLeg;
#[derive(Component)]
struct Body;
#[derive(Component)]
struct LeftArm;
#[derive(Component)]
struct RightArm;
#[derive(Component)]
struct Head;

fn add_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let sector_count = 32;
    let stack_count = 16;
    // ------------- Floor ------------------

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: meshes.add(Mesh::from(Cylinder::new(32, 0.1, 4., 4.))),
        transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        color: Color::BLACK,
        ..Default::default()
    });

    // ------------- Foot ------------------

    commands
        .spawn()
        .insert(Robot)
        .insert_bundle(TransformBundle {
            transform: Transform {
                translation: vec3(0., 0.0, 0.),
                ..Default::default()
            },
            global_transform: GlobalTransform::default(),
        })
        .with_children(|robot| {
            robot
                .spawn()
                .insert(RightLeg)
                .insert_bundle(TransformBundle {
                    transform: Transform {
                        translation: vec3(-1., 0.0, -0.25),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|right_leg| {
                    right_leg
                        .spawn()
                        .insert(LowerRightLeg)
                        .insert_bundle(TransformBundle::default())
                        .with_children(|lower_right_leg| {
                            lower_right_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 0.3, 1.))),
                                transform: Transform::from_xyz(0., 0.2, 0.25),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                            lower_right_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 2., 0.5))),
                                transform: Transform::from_xyz(0., 1., 0.),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                        });
                    right_leg
                        .spawn()
                        .insert(UpperRightLeg)
                        .insert_bundle(TransformBundle::default())
                        .with_children(|upper_right_leg| {
                            upper_right_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Sphere::new(
                                    sector_count,
                                    stack_count,
                                    0.45,
                                ))),
                                transform: Transform::from_xyz(0., 2.1, 0.),
                                color: Color::DARK_GRAY,
                                ..Default::default()
                            });
                            upper_right_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 1.8, 0.5))),
                                transform: Transform::from_xyz(0., 3., 0.),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                        });
                });
            robot
                .spawn()
                .insert(LeftLeg)
                .insert_bundle(TransformBundle {
                    transform: Transform {
                        translation: vec3(1., 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|right_leg| {
                    right_leg
                        .spawn()
                        .insert(LowerLeftLeg)
                        .insert_bundle(TransformBundle::default())
                        .with_children(|lower_left_leg| {
                            lower_left_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 0.3, 1.))),
                                transform: Transform::from_xyz(0., 0.2, 0.25),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                            lower_left_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 2., 0.5))),
                                transform: Transform::from_xyz(0., 1., 0.),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                        });
                    right_leg
                        .spawn()
                        .insert(UpperLeftLeg)
                        .insert_bundle(TransformBundle::default())
                        .with_children(|upper_left_leg| {
                            upper_left_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Sphere::new(
                                    sector_count,
                                    stack_count,
                                    0.45,
                                ))),
                                transform: Transform::from_xyz(0., 2.1, 0.),
                                color: Color::DARK_GRAY,
                                ..Default::default()
                            });
                            upper_left_leg.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 1.8, 0.5))),
                                transform: Transform::from_xyz(0., 3., 0.),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                        });
                });
            robot
                .spawn()
                .insert(Body)
                .insert_bundle(TransformBundle {
                    transform: Transform {
                        translation: vec3(0., 5.3, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|body| {
                    // trunk
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(2.55, 3.2, 1.))),
                        transform: Transform::from_xyz(0., 0., 0.),
                        color: Color::SILVER,
                        ..Default::default()
                    });
                    // muscles
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.9, 0.9, 0.1))),
                        transform: Transform::from_xyz(0.5, 0.9, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.9, 0.9, 0.1))),
                        transform: Transform::from_xyz(-0.5, 0.9, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.45, 0.1))),
                        transform: Transform::from_xyz(-0.25, -0.1, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.45, 0.1))),
                        transform: Transform::from_xyz(0.25, -0.1, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.45, 0.1))),
                        transform: Transform::from_xyz(-0.25, -0.6, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.45, 0.1))),
                        transform: Transform::from_xyz(0.25, -0.6, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.45, 0.1))),
                        transform: Transform::from_xyz(-0.25, -1.1, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    body.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.45, 0.1))),
                        transform: Transform::from_xyz(0.25, -1.1, 0.5),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    // head
                    body.spawn()
                        .insert(Head)
                        .insert_bundle(TransformBundle {
                            transform: Transform {
                                translation: vec3(0.0, 2.3, 0.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|head| {
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cylinder::new(
                                    sector_count,
                                    0.3,
                                    0.3,
                                    0.3,
                                ))),
                                transform: Transform {
                                    translation: vec3(0., -0.7, 0.),
                                    rotation: Quat::from_rotation_x(FRAC_PI_2),
                                    ..Default::default()
                                },
                                color: Color::DARK_GRAY,
                                ..Default::default()
                            });
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(1.4, 1.2, 0.7))),
                                transform: Transform::from_xyz(0., 0., 0.),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cylinder::new(
                                    sector_count,
                                    0.33,
                                    0.05,
                                    0.05,
                                ))),
                                transform: Transform::from_xyz(-0.25, 0.3, 0.35),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cylinder::new(
                                    sector_count,
                                    0.33,
                                    0.05,
                                    0.05,
                                ))),
                                transform: Transform::from_xyz(0.25, 0.3, 0.35),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cylinder::new(
                                    sector_count,
                                    0.33,
                                    0.05,
                                    0.05,
                                ))),
                                transform: Transform::from_xyz(0.25, 0.3, 0.35),
                                color: Color::SILVER,
                                ..Default::default()
                            });
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cylinder::new(
                                    sector_count,
                                    0.3,
                                    0.1,
                                    0.1,
                                ))),
                                transform: Transform::from_xyz(-0.25, 0.3, 0.35),
                                color: Color::DARK_GRAY,
                                ..Default::default()
                            });
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cylinder::new(
                                    sector_count,
                                    0.3,
                                    0.1,
                                    0.1,
                                ))),
                                transform: Transform::from_xyz(0.25, 0.3, 0.35),
                                color: Color::DARK_GRAY,
                                ..Default::default()
                            });
                            head.spawn_bundle(SimpleMeshBundle {
                                mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 0.2, 0.1))),
                                transform: Transform::from_xyz(0.0, -0.2, 0.35),
                                color: Color::DARK_GRAY,
                                ..Default::default()
                            });
                        });
                });
            robot
                .spawn()
                .insert(RightArm)
                .insert_bundle(TransformBundle {
                    transform: Transform {
                        translation: vec3(1.6, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|right_arm| {
                    right_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 2., 0.5))),
                        transform: Transform::from_xyz(0., 5.6, 0.),
                        color: Color::SILVER,
                        ..Default::default()
                    });
                    right_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Sphere::new(sector_count, stack_count, 0.45))),
                        transform: Transform::from_xyz(0., 6.6, 0.),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    right_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Sphere::new(sector_count, stack_count, 0.45))),
                        transform: Transform::from_xyz(0., 4.5, 0.),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    right_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 1.3, 0.5))),
                        transform: Transform::from_xyz(0., 3.7, 0.),
                        color: Color::SILVER,
                        ..Default::default()
                    });
                    right_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.55, 0.35))),
                        transform: Transform::from_xyz(0., 3., 0.3),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    right_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.55, 0.35))),
                        transform: Transform::from_xyz(0., 3., -0.3),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                });
            robot
                .spawn()
                .insert(LeftArm)
                .insert_bundle(TransformBundle {
                    transform: Transform {
                        translation: vec3(-1.6, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|left_arm| {
                    left_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 2., 0.5))),
                        transform: Transform::from_xyz(0., 5.6, 0.),
                        color: Color::SILVER,
                        ..Default::default()
                    });
                    left_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Sphere::new(sector_count, stack_count, 0.45))),
                        transform: Transform::from_xyz(0., 6.6, 0.),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    left_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Sphere::new(sector_count, stack_count, 0.45))),
                        transform: Transform::from_xyz(0., 4.5, 0.),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    left_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 1.3, 0.5))),
                        transform: Transform::from_xyz(0., 3.7, 0.),
                        color: Color::SILVER,
                        ..Default::default()
                    });
                    left_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.55, 0.35))),
                        transform: Transform::from_xyz(0., 3., 0.3),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    left_arm.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.55, 0.35))),
                        transform: Transform::from_xyz(0., 3., -0.3),
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                });
        });
}

fn add_camera(mut commands: Commands) {
    // ------------ Camera -----------------
    commands.spawn_bundle(FlyCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at_xyz(0.0, 4.0, 0.0),
        ..Default::default()
    });
}
