use glium::Surface;
use bevy_ecs::prelude::*;
use crate::{ClearColor, Color};
use arara_camera::camera_controller::CameraController;
use arara_geometry::Sphere;
use arara_shaders::Shaders;
use arara_transform::Transform;
use arara_window::Window;


pub fn draw(
    window: NonSend<Window>,
    clear_color: Res<ClearColor>,
    mut camera_controller: ResMut<CameraController>,
    query: Query<(&Sphere, &Shaders, &Transform, &Color)>,
) {
    let display = window.display();
    let clear_color = clear_color.0;

    let u_pv_matrix = camera_controller.calc_matrix();
    let light = [50.0, 50.0, 50.0f32];
    
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let mut frame = display.draw();
    frame.clear_color_and_depth((clear_color.r(), clear_color.g(), clear_color.b(), clear_color.a()), 1.0);
    for (shape, shaders, transform, color) in query.iter() {
        let vertex_buffer = glium::VertexBuffer::new(display, &shape.vertices).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &shape.indices).unwrap();
        let program = glium::Program::from_source(display, shaders.vertex_shader, shaders.fragment_shader, None).unwrap();
        let color: [f32; 3] = color.to_owned().into();
        
        let uniforms = uniform! {
            u_pv_matrix: u_pv_matrix,
            transform: transform.transform,
            u_light: light,
            u_color: color,
        };
        
        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
    }
    frame.finish().unwrap();
}