use arara::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        // .add_plugin(CoordinateSystemPlugin)
        .insert_resource(CoordinateSystem {
            count: 10,
            lenght: 10.0,
            radius: 1.0,
        })
        .add_plugin(LogDiagnosticPlugin {
            wait_duration: Duration::from_secs(3),
        })
        .add_startup_system(add_shapes.system())
        .insert_resource(BPLight {
            position: vec3(5.0, 30.0, 55.0),
        })
        // .insert_resource(Camera::new(vec3(0.0, 30.0, 70.0), -FRAC_PI_2, -FRAC_PI_6))
        // .insert_resource(FlyCamera::new(20.0, 0.5))
        .run()
}

#[derive(Component)]
struct Tower;
#[derive(Component)]
struct Wall;

fn add_shapes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // ------------ Camera -----------------
    commands.spawn_bundle(FlyCameraBundle {
        transform: Transform::from_xyz(5.0, 30.0, 55.0).looking_at_xyz(0.0, 10.0, 0.0),
        ..Default::default()
    });

    // ------------- Floor ------------------
    let floor_height = 10.0;
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: meshes.add(Mesh::from(Cylinder::new(32, floor_height, 42., 50.))),
        transform: Transform {
            translation: vec3(0.0, -floor_height / 2.0, 0.0),
            rotation: Quat::from_rotation_x(FRAC_PI_2),
            ..Default::default()
        },
        color: Color::DARK_GREEN,
        ..Default::default()
    });

    // ------------- Tower ------------------
    let tower_radius = 5.0;
    let tower_distance = 50.0;
    let tower_height = 10.0;

    let tower_mesh = meshes.add(Mesh::from(Cylinder::new(
        16,
        tower_height,
        tower_radius,
        tower_radius,
    )));
    let block_width = 1.0;
    let block_mesh = meshes.add(Mesh::from(Cuboid::new(block_width, 2.0, block_width)));

    for i in 0..2 {
        for j in 0..2 {
            let x = i as f32 * tower_distance - tower_distance / 2.0;
            let z = j as f32 * tower_distance - tower_distance / 2.0;
            commands
                .spawn()
                .insert(Tower)
                .insert_bundle(TransformBundle::default())
                .with_children(|tower| {
                    tower.spawn_bundle(SimpleMeshBundle {
                        mesh: tower_mesh.clone(),
                        transform: Transform {
                            translation: vec3(x, tower_height / 2.0, z),
                            rotation: Quat::from_rotation_x(FRAC_PI_2),
                            ..Default::default()
                        },
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    let amount = 20;
                    let step_angle = 2.0 * PI / amount as f32;
                    for i in 0..amount {
                        let block_angle = i as f32 * step_angle;
                        tower.spawn_bundle(SimpleMeshBundle {
                            mesh: block_mesh.clone(),
                            transform: Transform::from_xyz(
                                x + block_angle.cos() * tower_radius,
                                tower_height,
                                z + block_angle.sin() * tower_radius,
                            ),
                            color: Color::DARK_GRAY,
                            ..Default::default()
                        });
                    }
                });
        }
    }

    // ------------- Walls ------------------
    let wall_thickness = 3.0;
    let wall_height = tower_height * 0.7;
    let wall_lenght = tower_distance;
    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(SimpleMeshBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                wall_lenght,
                wall_height,
                wall_thickness,
            ))),
            transform: Transform::from_xyz(0.0, wall_height / 2.0, -tower_distance / 2.0),
            color: Color::DARK_GRAY,
            ..Default::default()
        });

    let wall_mesh = meshes.add(Mesh::from(Cuboid::new(
        wall_thickness,
        wall_height,
        wall_lenght,
    )));

    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(SimpleMeshBundle {
            mesh: wall_mesh.clone(),
            transform: Transform::from_xyz(-tower_distance / 2.0, wall_height / 2.0, 0.0),
            color: Color::DARK_GRAY,
            ..Default::default()
        });
    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(SimpleMeshBundle {
            mesh: wall_mesh.clone(),
            transform: Transform::from_xyz(tower_distance / 2.0, wall_height / 2.0, 0.0),
            color: Color::DARK_GRAY,
            ..Default::default()
        });

    // ------------- Gate ------------------
    let gate_lenght = 5.0;
    let gate_height = 3.0;
    let gate_wall_lenght = (wall_lenght - gate_lenght) / 2.0;

    let wall_mesh = meshes.add(Mesh::from(Cuboid::new(
        gate_wall_lenght,
        wall_height,
        wall_thickness,
    )));

    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(SimpleMeshBundle {
            mesh: wall_mesh.clone(),
            transform: Transform::from_xyz(
                -(gate_wall_lenght + gate_lenght) / 2.0,
                wall_height / 2.0,
                tower_distance / 2.0,
            ),
            color: Color::DARK_GRAY,
            ..Default::default()
        });
    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(SimpleMeshBundle {
            mesh: wall_mesh.clone(),
            transform: Transform::from_xyz(
                (gate_wall_lenght + gate_lenght) / 2.0,
                wall_height / 2.0,
                tower_distance / 2.0,
            ),
            color: Color::DARK_GRAY,
            ..Default::default()
        });
    commands
        .spawn()
        .insert(Wall)
        .insert_bundle(SimpleMeshBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                gate_lenght,
                wall_height - gate_height,
                wall_thickness,
            ))),
            transform: Transform::from_xyz(
                0.0,
                (wall_height - gate_height) / 2.0 + gate_height,
                tower_distance / 2.0,
            ),
            color: Color::DARK_GRAY,
            ..Default::default()
        });

    // ------------- Houses ------------------

    let house_width = 5.0;
    let house_height = 3.0;
    let house_spacing = 3.0;
    let roof_height = 2.0;
    let house_body_mesh = meshes.add(Mesh::from(Cuboid::new(
        house_width,
        house_height,
        house_width,
    )));
    let house_roof_mesh = meshes.add(Mesh::from(Cylinder::new(
        4,
        roof_height,
        house_width * 1.01,
        0.0,
    )));

    commands
        .spawn()
        .insert_bundle(TransformBundle {
            transform: Transform::from_xyz(-20., 0.0, -20.),
            ..Default::default()
        })
        .with_children(|pivot| {
            for i in 0..2 {
                for j in 0..5 {
                    let x = i as f32 * (house_spacing + house_width) + house_width / 2.0;
                    let z = j as f32 * (house_spacing + house_width) + house_width / 2.0;
                    pivot.spawn_bundle(SimpleMeshBundle {
                        mesh: house_body_mesh.clone(),
                        transform: Transform::from_xyz(x, house_height / 2.0, z),
                        color: Color::WHITE,
                        ..Default::default()
                    });
                    pivot.spawn_bundle(SimpleMeshBundle {
                        mesh: house_roof_mesh.clone(),
                        transform: Transform {
                            translation: vec3(x, house_height + roof_height / 2.0, z),
                            // rotation: Quat::from_euler(EulerRot::XYZ, 0.0, FRAC_PI_2, FRAC_PI_2),
                            rotation: Quat::from_euler(
                                EulerRot::ZYX,
                                0.0,
                                FRAC_PI_2 / 2.0,
                                -FRAC_PI_2,
                            ),
                            ..Default::default()
                        },
                        color: Color::ORANGE,
                        ..Default::default()
                    });
                }
            }
        });

    // ------------- House ------------------
    let house_width = 20.0;
    let house_height = 5.0;
    commands
        .spawn()
        .insert_bundle(SimpleMeshBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                house_width,
                house_height,
                house_width,
            ))),
            transform: Transform::from_xyz(10.0, house_height / 2.0, 0.0),
            color: Color::WHITE,
            ..Default::default()
        })
        .with_children(|tower| {
            let tower_radius = 2.0;
            let tower_distance = house_width;
            let tower_height = house_height * 1.2;
            let cap_height = 2.0;

            let tower_body_mesh = meshes.add(Mesh::from(Cylinder::new(
                6,
                tower_height,
                tower_radius,
                tower_radius,
            )));
            let tower_cap_mesh = meshes.add(Mesh::from(Cylinder::new(
                6,
                cap_height,
                tower_radius * 1.1,
                0.0,
            )));

            for i in 0..2 {
                for j in 0..2 {
                    let x = i as f32 * tower_distance - tower_distance / 2.0;
                    let z = j as f32 * tower_distance - tower_distance / 2.0;
                    tower.spawn_bundle(SimpleMeshBundle {
                        mesh: tower_body_mesh.clone(),
                        transform: Transform {
                            translation: vec3(x, 0.0, z),
                            rotation: Quat::from_rotation_x(FRAC_PI_2),
                            ..Default::default()
                        },
                        color: Color::DARK_GRAY,
                        ..Default::default()
                    });
                    tower.spawn_bundle(SimpleMeshBundle {
                        mesh: tower_cap_mesh.clone(),
                        transform: Transform {
                            translation: vec3(x, tower_height / 2.0 + cap_height / 2.0, z),
                            rotation: Quat::from_rotation_x(-FRAC_PI_2),
                            ..Default::default()
                        },
                        color: Color::RED,
                        ..Default::default()
                    });
                }
            }
        })
        .with_children(|top| {
            let house_width = 0.5 * house_width;
            let house_height = 5.0;
            top.spawn()
                .insert_bundle(SimpleMeshBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(
                        house_width,
                        house_height,
                        house_width,
                    ))),
                    transform: Transform::from_xyz(0.0, house_height, 0.0),
                    color: Color::WHITE,
                    ..Default::default()
                })
                .with_children(|tower| {
                    let tower_radius = 2.0;
                    let tower_distance = house_width;
                    let tower_height = house_height * 1.2;
                    let cap_height = 2.0;

                    let tower_mesh = meshes.add(Mesh::from(Cylinder::new(
                        6,
                        tower_height,
                        tower_radius,
                        tower_radius,
                    )));
                    let tower_cap = meshes.add(Mesh::from(Cylinder::new(
                        6,
                        cap_height,
                        tower_radius * 1.1,
                        0.0,
                    )));

                    for i in 0..2 {
                        for j in 0..2 {
                            let x = i as f32 * tower_distance - tower_distance / 2.0;
                            let z = j as f32 * tower_distance - tower_distance / 2.0;
                            tower.spawn_bundle(SimpleMeshBundle {
                                mesh: tower_mesh.clone(),
                                transform: Transform {
                                    translation: vec3(x, 0.0, z),
                                    rotation: Quat::from_rotation_x(FRAC_PI_2),
                                    ..Default::default()
                                },
                                color: Color::DARK_GRAY,
                                ..Default::default()
                            });
                            tower.spawn_bundle(SimpleMeshBundle {
                                mesh: tower_cap.clone(),
                                transform: Transform {
                                    translation: vec3(x, tower_height / 2.0 + cap_height / 2.0, z),
                                    rotation: Quat::from_rotation_x(-FRAC_PI_2),
                                    ..Default::default()
                                },
                                color: Color::RED,
                                ..Default::default()
                            });
                        }
                    }
                })
                .with_children(|toptop| {
                    let tower_radius = 2.5;
                    let tower_height = house_height * 0.6;
                    let cap_height = 2.0;
                    toptop.spawn().insert_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cylinder::new(
                            6,
                            tower_height,
                            tower_radius,
                            tower_radius,
                        ))),
                        transform: Transform {
                            translation: vec3(0.0, house_height / 2.0 + tower_height / 2.0, 0.0),
                            rotation: Quat::from_rotation_x(FRAC_PI_2),
                            ..Default::default()
                        },
                        color: Color::GRAY,
                        ..Default::default()
                    });
                    toptop.spawn_bundle(SimpleMeshBundle {
                        mesh: meshes.add(Mesh::from(Cylinder::new(
                            6,
                            cap_height,
                            tower_radius * 1.5,
                            0.0,
                        ))),
                        transform: Transform {
                            translation: vec3(
                                0.0,
                                house_height / 2.0 + tower_height + cap_height / 2.0,
                                0.0,
                            ),
                            rotation: Quat::from_rotation_x(-FRAC_PI_2),
                            ..Default::default()
                        },
                        color: Color::GOLD,
                        ..Default::default()
                    });
                });
        });
}
