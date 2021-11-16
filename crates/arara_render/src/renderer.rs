use glium::Surface;
use bevy_ecs::prelude::*;
use crate::Color;
use arara_camera::camera_controller::CameraController;
use arara_geometry::Shape;
use arara_shaders::Shaders;
use arara_transform::Transform;
use arara_window::Window;


pub fn draw(
    window: NonSend<Window>,
    mut camera_controller: ResMut<CameraController>,
    query: Query<(&Box<dyn Shape>, &Shaders, &Transform, &Color)>,
) {
    let display = window.display();

    let u_pv_matrix = camera_controller.calc_matrix();
    let light = [0.0, 0.0, 0.0f32];
    
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
    frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
    for (shape, shaders, transform, color) in query.iter() {
        let vertex_buffer = glium::VertexBuffer::new(display, &shape.get_vertices()).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &shape.get_indices()).unwrap();
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