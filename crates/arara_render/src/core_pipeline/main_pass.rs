use crate::{
    geometry::Mesh, render_phase::RenderPhase, BPLight, ClearColor, ExtractedCPE, ExtractedView,
    Opaque, RenderPipelineCache, TextureBuffer, Transparent,
};
use arara_asset::Assets;
use arara_ecs::prelude::*;
use arara_window::Window;
use glam::*;
use glium::{implement_uniform_block, implement_vertex, Surface};

#[cfg(feature = "trace")]
use arara_utils::tracing::info_span;

#[derive(Copy, Clone)]
struct Vertex {
    i_position: [f32; 3],
    i_normal: [f32; 3],
    i_color: [f32; 4],
    i_tex_cords: [f32; 2],
    i_tex_id: u32,
}

implement_vertex!(Vertex, i_position, i_normal, i_color, i_tex_cords, i_tex_id);

#[derive(Debug, Default, Clone, Copy)]
struct CameraUniformBuffer {
    u_pv_matrix: [[f32; 4]; 4],
}

impl CameraUniformBuffer {
    fn new(u_pv_matrix: [[f32; 4]; 4]) -> Self {
        Self { u_pv_matrix }
    }
}

implement_uniform_block!(CameraUniformBuffer, u_pv_matrix);

#[derive(Copy, Clone)]
struct TextureUniformBuffer<'a> {
    tex: [glium::texture::TextureHandle<'a>; 5],
}

implement_uniform_block!(TextureUniformBuffer<'a>, tex);

#[derive(Copy, Clone)]
struct BPLightUniformBuffer {
    pub u_camera_pos: [f32; 3],
    pub u_light_pos: [f32; 3],
}

implement_uniform_block!(BPLightUniformBuffer, u_camera_pos, u_light_pos);

pub fn main_pass(
    window: NonSend<Window>,
    clear_color: Res<ClearColor>,
    light: Res<BPLight>,
    view: Res<ExtractedView>,
    texture_buffer: NonSend<TextureBuffer>,
    meshes: Res<Assets<Mesh>>,
    opaques: Res<RenderPhase<Opaque>>,
    transparents: Res<RenderPhase<Transparent>>,
    query: Query<&ExtractedCPE>,
    render_pipeline_cache: NonSend<RenderPipelineCache>,
) {
    let display = window.display();

    let clear_color = (
        clear_color.0.r(),
        clear_color.0.g(),
        clear_color.0.b(),
        clear_color.0.a(),
    );

    let pv_matrix: [[f32; 4]; 4] = view.pv_matrix.to_cols_array_2d();
    let camera_uniform_buffer =
        glium::uniforms::UniformBuffer::new(display, CameraUniformBuffer::new(pv_matrix)).unwrap();

    let texture_uniform_buffer =
        glium::uniforms::UniformBuffer::new(display, texture_buffer.texture_uniform_buffer())
            .unwrap();

    let bplight_uniform_buffer = glium::uniforms::UniformBuffer::new(
        display,
        BPLightUniformBuffer {
            u_camera_pos: view.position.into(),
            u_light_pos: light.position.into(),
        },
    )
    .unwrap();

    // Start Frame
    let mut frame = display.draw();
    frame.clear_color_and_depth(clear_color, 1.0);

    // Main pass
    if !opaques.items.is_empty() {
        #[cfg(feature = "trace")]
        let opaque_run_span = info_span!("arara_render::opaque");
        #[cfg(feature = "trace")]
        let _opaque_run_guard = opaque_run_span.enter();

        let pipeline = opaques.items.first().unwrap();
        let pipeline = match render_pipeline_cache.get(pipeline.pipeline) {
            Some(pipeline) => pipeline,
            None => {
                frame.finish().unwrap();
                return;
            }
        };

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for opaque in opaques.items.iter() {
            let ExtractedCPE {
                mesh: mesh_handle,
                transform,
                color,
                image: image_handle,
            } = query.get(opaque.entity).unwrap();

            let tex_id = texture_buffer.get_position(image_handle);

            let mesh = meshes.get(mesh_handle).unwrap();
            let offset = vertices.len() as u32;
            let ti_transform = Mat3::from_mat4(transform.inverse().transpose());
            let color: [f32; 4] = color.to_owned().into();

            for vertex in mesh.vertices.iter() {
                let position = vec4(
                    vertex.position[0],
                    vertex.position[1],
                    vertex.position[2],
                    1.0,
                );
                let position = *transform * position;
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

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        let uniforms = glium::uniform! {
            camera: &camera_uniform_buffer,
            bplight: &bplight_uniform_buffer,
            samplers: &texture_uniform_buffer,
        };

        #[cfg(feature = "trace")]
        let opaque_draw_run_span = info_span!("arara_render::opaque::draw_call");
        #[cfg(feature = "trace")]
        let _opaque_draw_run_guard = opaque_draw_run_span.enter();

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &pipeline.program,
                &uniforms,
                &pipeline.parameters,
            )
            .unwrap();
    }

    // Translucent pass
    if !transparents.items.is_empty() {
        #[cfg(feature = "trace")]
        let transparent_run_span = info_span!("arara_render::transparent");
        #[cfg(feature = "trace")]
        let _transparent_run_guard = transparent_run_span.enter();

        let pipeline = transparents.items.first().unwrap();
        let pipeline = match render_pipeline_cache.get(pipeline.pipeline) {
            Some(pipeline) => pipeline,
            None => {
                frame.finish().unwrap();
                return;
            }
        };

        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        for transparent in transparents.items.iter() {
            let ExtractedCPE {
                mesh: mesh_handle,
                transform,
                color,
                image: image_handle,
            } = query.get(transparent.entity).unwrap();

            let tex_id = texture_buffer.get_position(image_handle);

            let mesh = meshes.get(mesh_handle).unwrap();
            let offset = vertices.len() as u32;
            let ti_transform = Mat3::from_mat4(transform.inverse().transpose());
            let color: [f32; 4] = color.to_owned().into();

            for vertex in mesh.vertices.iter() {
                let position = vec4(
                    vertex.position[0],
                    vertex.position[1],
                    vertex.position[2],
                    1.0,
                );
                let position = *transform * position;
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

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        let uniforms = glium::uniform! {
            camera: &camera_uniform_buffer,
            bplight: &bplight_uniform_buffer,
            samplers: &texture_uniform_buffer,
        };

        #[cfg(feature = "trace")]
        let transparent_draw_run_span = info_span!("arara_render::transparent::draw_call");
        #[cfg(feature = "trace")]
        let _transparent_draw_run_guard = transparent_draw_run_span.enter();

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &pipeline.program,
                &uniforms,
                &pipeline.parameters,
            )
            .unwrap();
    }

    // Finish Frame
    frame.finish().unwrap();
}
