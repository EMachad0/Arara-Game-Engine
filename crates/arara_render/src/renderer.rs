use glam::*;
use glium::*;
use glium::texture::RawImage2d;
use bevy_ecs::prelude::*;
use image::DynamicImage;
use crate::{ClearColor, Color, BPLight};
use arara_camera::FlyCamera;
use arara_geometry::Shape;
use arara_shaders::Shaders;
use arara_transform::{Transform, GlobalTransform};
use arara_window::Window;

#[derive(Copy, Clone)]
struct Vertex {
    i_position: [f32; 3],
    i_normal: [f32; 3],
    i_color: [f32; 4],
    i_tex_cords: [f32; 2],
    i_tex_id: u32,
}

implement_vertex!(Vertex, i_position, i_normal, i_color, i_tex_cords, i_tex_id);


pub fn draw(
    window: NonSend<Window>,
    clear_color: Res<ClearColor>,
    light: Res<BPLight>,
    mut camera_controller: ResMut<FlyCamera>,
    query: Query<(&Box<dyn Shape>, &Transform, &GlobalTransform, &Color, &Option::<DynamicImage>)>,
) {
    let display = window.display();
    let clear_color = clear_color.0;

    let pv_matrix = camera_controller.calc_matrix();
    let camera_pos: [f32; 3] = camera_controller.camera.position.into();
    let light_pos: [f32; 3] = light.position.into();
    
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        // line_width: Some(2.0),
        // polygon_mode: glium::PolygonMode::Line,
        ..Default::default()
    };

    let shaders = Shaders::default();
    let program = glium::Program::from_source(display, shaders.vertex_shader, shaders.fragment_shader, None).unwrap();
    
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut images: Vec<RawImage2d::<u8>> = vec![
        RawImage2d::from_raw_rgb(vec![255; 3*1*1], (1, 1))
    ];
    
    for (shape, _transform, global_transform, color, dynamic_image) in query.iter() {
        let transform = global_transform.compute_matrix();
        let ti_transform = Mat3::from_mat4(global_transform.compute_matrix().inverse().transpose());
        let color: [f32; 4] = color.to_owned().into();
        let tex_id = match dynamic_image {
            Some(dynamic_image) => {
                let image = dynamic_image.to_rgba8();
                let image_dimensions = image.dimensions();
                let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
                images.push(image);
                images.len() - 1
            },
            None => 0,
        };
        let offset = vertices.len() as u32;

        for vertex in shape.get_vertices().iter() {
            let position = Vec4::new(vertex.position[0], vertex.position[1], vertex.position[2], 1.0);
            let position = transform * position;
            let normal = Vec3::new(vertex.normal[0], vertex.normal[1], vertex.normal[2]);
            let normal = ti_transform * normal;
            vertices.push(Vertex {
                i_position: [position.x, position.y, position.z],
                i_normal: normal.into(),
                i_color: color,
                i_tex_cords: vertex.tex_coords,
                i_tex_id: tex_id as u32,
            });
        }
        for idx in shape.get_indices().iter() {
            indices.push(*idx + offset);
        }
    }
    
    let texture_array = glium::texture::SrgbTexture2dArray::new(display, images).unwrap();
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    let uniforms = uniform! {
        u_pv_matrix: pv_matrix,
        u_light_pos: light_pos,
        u_camera_pos: camera_pos,
        u_texture_array: texture_array,
    };

    let mut frame = display.draw();
    frame.clear_color_and_depth((clear_color.r(), clear_color.g(), clear_color.b(), clear_color.a()), 1.0);
    frame.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params).unwrap();
    frame.finish().unwrap();
}