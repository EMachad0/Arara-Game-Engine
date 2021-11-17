use arara::prelude::*;
use arara_render::*;
use arara_diagnostic::prelude::*;
use arara_time::TimePlugin;
use cgmath::{Deg, Vector3};

fn main() {
    logger::init();

    App::builder()
        .add_plugin(TimePlugin)
        .add_plugin(WindowPlugin)
        .add_plugin(RenderPlugin)
        .add_plugin(DiagnosticsPlugin)
        .add_plugin(FrameTimeDiagnosticPlugin)
        .add_plugin(EntityCountDiagnosticPlugin)
        .add_plugin(LogDiagnosticPlugin::default())
        .init_resource::<CameraController>()
        .add_startup_system(add_shapes.system())
        .build()
        .run()
}

fn add_shapes(mut commands: Commands) {
    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 3.0, 2.0, 0.5)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(2.0, 1.5, 0.0)
            .build(),
        color: Color::PURPLE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(4, 1.0, 1.0, 1.0)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .translate(-1.0, 1.0, -1.0)
            .build(),
        color: Color::PURPLE,
    });

    commands.spawn_bundle(SimpleMeshBundle {
        mesh: Box::new(Cylinder::new(32, 1.0, 10.0, 10.0)),
        shaders: Shaders::default(),
        transform: TransformBuilder::new()
            .rotate(Vector3::unit_x(), Deg(75.0))
            .build(),
        color: Color::DARK_GREEN,
    });
}