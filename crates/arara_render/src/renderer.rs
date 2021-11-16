use glium::Surface;
use bevy_ecs::prelude::*;
use crate::{ClearColor, Color};
use arara_camera::camera_controller::CameraController;
use arara_geometry::Shape;
use arara_shaders::Shaders;
use arara_transform::Transform;
use arara_window::Window;


pub fn draw(
    window: NonSend<Window>,
    clear_color: Res<ClearColor>,
    mut camera_controller: ResMut<CameraController>,
    query: Query<(&Box<dyn Shape>, &Shaders, &Transform, &Color)>,
) {
    let display = window.display();
    let clear_color = clear_color.0;

    let pv_matrix = camera_controller.calc_matrix();
    let camera_pos: [f32; 3] = camera_controller.camera().position.into();
    let light_pos: [f32; 3] = [0.0, 500.0, 0.0];
    
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
        let vertex_buffer = glium::VertexBuffer::new(display, &shape.get_vertices()).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &shape.get_indices()).unwrap();
        let program = glium::Program::from_source(display, shaders.vertex_shader, shaders.fragment_shader, None).unwrap();
        let color: [f32; 3] = color.to_owned().into();
        
        let uniforms = uniform! {
            u_pv_matrix: pv_matrix,
            u_transform: transform.transform,
            u_light_pos: light_pos,
            u_camera_pos: camera_pos,
            u_color: color,
        };
        
        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
    }
    frame.finish().unwrap();
}