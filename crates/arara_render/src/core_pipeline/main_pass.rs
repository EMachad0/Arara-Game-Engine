use crate::{
    geometry::Mesh, render_phase::RenderPhase, BPLight, ClearColor, Color, DefaultShader, Image,
    Opaque, Shader, Transparent,
};
use arara_asset::{Assets, Handle};
use arara_camera::{Camera, Perspective};
use arara_ecs::prelude::*;
use arara_transform::GlobalTransform;
use arara_utils::StableHashMap;
use arara_window::Window;
use glam::*;
use glium::{texture::RawImage2d, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    i_position: [f32; 3],
    i_normal: [f32; 3],
    i_color: [f32; 4],
    i_tex_cords: [f32; 2],
    i_tex_id: u32,
}

glium::implement_vertex!(Vertex, i_position, i_normal, i_color, i_tex_cords, i_tex_id);

pub fn main_pass(
    window: NonSend<Window>,
    clear_color: Res<ClearColor>,
    light: Res<BPLight>,
    camera: Res<Camera>,
    perspective: Res<Perspective>,
    default_shader: Res<DefaultShader>,
    images: Res<Assets<Image>>,
    meshes: Res<Assets<Mesh>>,
    shaders: Res<Assets<Shader>>,
    opaques: Res<RenderPhase<Opaque>>,
    transparents: Res<RenderPhase<Transparent>>,
    query: Query<(&Handle<Mesh>, &GlobalTransform, &Color, &Handle<Image>)>,
) {
    let display = window.display();
    let clear_color = clear_color.0;
    let clear_color = (
        clear_color.r(),
        clear_color.g(),
        clear_color.b(),
        clear_color.a(),
    );

    let pv_matrix: [[f32; 4]; 4] = (perspective.calc_matrix() * camera.calc_matrix()).to_cols_array_2d();
    let camera_pos: [f32; 3] = camera.position.into();
    let light_pos: [f32; 3] = light.position.into();

    let DefaultShader {
        vertex_shader,
        fragment_shader,
    } = &*default_shader;
    let vertex_shader = shaders.get(vertex_shader);
    let fragment_shader = shaders.get(fragment_shader);
    if vertex_shader.is_none() || fragment_shader.is_none() {
        return;
    }
    let program = glium::Program::from_source(
        display,
        vertex_shader.unwrap().source(),
        fragment_shader.unwrap().source(),
        None,
    )
    .unwrap();

    // Start Frame
    let mut frame = display.draw();
    frame.clear_color_and_depth(clear_color, 1.0);

    // Main pass
    if !opaques.items.is_empty() {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut textures: Vec<RawImage2d<u8>> = vec![RawImage2d::from_raw_rgba_reversed(
            &vec![255; 4 * 64 * 64],
            (64, 64),
        )];

        let mut textures_index: StableHashMap<Handle<Image>, u32> = StableHashMap::default();

        for opaque in opaques.items.iter() {
            let (mesh, global_transform, color, image_handle) = query.get(opaque.entity).unwrap();

            let tex_id = if *image_handle == Handle::<Image>::default() {
                0
            } else {
                match textures_index.get(image_handle) {
                    Some(index) => index.to_owned(),
                    None => match images.get(image_handle) {
                        Some(image) => {
                            let texture =
                                RawImage2d::from_raw_rgba_reversed(&image.data, image.dimensions);
                            let index = textures.len() as u32;
                            textures.push(texture);
                            textures_index.insert(image_handle.clone(), index);
                            index
                        }
                        None => continue,
                    },
                }
            };

            let mesh = meshes.get(mesh).unwrap();
            let offset = vertices.len() as u32;
            let transform = global_transform.compute_matrix();
            let ti_transform =
                Mat3::from_mat4(global_transform.compute_matrix().inverse().transpose());
            let color: [f32; 4] = color.to_owned().into();

            for vertex in mesh.vertices.iter() {
                let position = vec4(
                    vertex.position[0],
                    vertex.position[1],
                    vertex.position[2],
                    1.0,
                );
                let position = transform * position;
                let normal = vec3(vertex.normal[0], vertex.normal[1], vertex.normal[2]);
                let normal = ti_transform * normal;
                vertices.push(Vertex {
                    i_position: [position.x, position.y, position.z],
                    i_normal: normal.into(),
                    i_color: color,
                    i_tex_cords: vertex.tex_coords,
                    i_tex_id: tex_id,
                });
            }
            for idx in mesh.indices.iter() {
                indices.push(*idx + offset);
            }
        }

        let texture_array = glium::texture::SrgbTexture2dArray::new(display, textures).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        let uniforms = glium::uniform! {
            u_pv_matrix: pv_matrix,
            u_light_pos: light_pos,
            u_camera_pos: camera_pos,
            u_texture_array: texture_array,
        };

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
        frame
            .draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params)
            .unwrap();
    }

    // Translucent pass
    if !transparents.items.is_empty() {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut textures: Vec<RawImage2d<u8>> = vec![RawImage2d::from_raw_rgba_reversed(
            &vec![255; 4 * 64 * 64],
            (64, 64),
        )];

        let mut textures_index: StableHashMap<Handle<Image>, u32> = StableHashMap::default();

        for transparent in transparents.items.iter() {
            let (mesh, global_transform, color, image_handle) =
                query.get(transparent.entity).unwrap();

            let tex_id = if *image_handle == Handle::<Image>::default() {
                0
            } else {
                match textures_index.get(image_handle) {
                    Some(index) => index.to_owned(),
                    None => match images.get(image_handle) {
                        Some(image) => {
                            let texture =
                                RawImage2d::from_raw_rgba_reversed(&image.data, image.dimensions);
                            let index = textures.len() as u32;
                            textures.push(texture);
                            textures_index.insert(image_handle.clone(), index);
                            index
                        }
                        None => continue,
                    },
                }
            };

            let mesh = meshes.get(mesh).unwrap();
            let offset = vertices.len() as u32;
            let transform = global_transform.compute_matrix();
            let ti_transform =
                Mat3::from_mat4(global_transform.compute_matrix().inverse().transpose());
            let color: [f32; 4] = color.to_owned().into();

            for vertex in mesh.vertices.iter() {
                let position = vec4(
                    vertex.position[0],
                    vertex.position[1],
                    vertex.position[2],
                    1.0,
                );
                let position = transform * position;
                let normal = vec3(vertex.normal[0], vertex.normal[1], vertex.normal[2]);
                let normal = ti_transform * normal;
                vertices.push(Vertex {
                    i_position: [position.x, position.y, position.z],
                    i_normal: normal.into(),
                    i_color: color,
                    i_tex_cords: vertex.tex_coords,
                    i_tex_id: tex_id,
                });
            }
            for idx in mesh.indices.iter() {
                indices.push(*idx + offset);
            }
        }

        let texture_array = glium::texture::SrgbTexture2dArray::new(display, textures).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        let uniforms = glium::uniform! {
            u_pv_matrix: pv_matrix,
            u_light_pos: light_pos,
            u_camera_pos: camera_pos,
            u_texture_array: texture_array,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                //     write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..Default::default()
        };
        frame
            .draw(&vertex_buffer, &index_buffer, &program, &uniforms, &params)
            .unwrap();
    }

    // Finish Frame
    frame.finish().unwrap();
}
