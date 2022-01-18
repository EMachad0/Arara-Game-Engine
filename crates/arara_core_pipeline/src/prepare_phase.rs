use arara_asset::{Assets, Handle};
use arara_ecs::prelude::*;
use arara_render::{Image, Mesh, TextureBuffer};
use arara_window::Window;
use glam::{vec3, vec4, Mat3};

use crate::core_pipeline_entities::ExtractedCorePipelineEntity;

#[derive(Copy, Clone)]
pub struct Vertex {
    i_position: [f32; 3],
    i_normal: [f32; 3],
    i_color: [f32; 4],
    i_tex_cords: [f32; 2],
    i_tex_id: u32,
}

glium::implement_vertex!(Vertex, i_position, i_normal, i_color, i_tex_cords, i_tex_id);

#[derive(Component)]
pub struct CorePipelineBatch {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub transparent: bool,
}

pub(crate) fn prepare_core_pipeline_phase(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    query: Query<&ExtractedCorePipelineEntity>,
    texture_buffer: NonSend<TextureBuffer>,
) {
    let mut vertices_opaque = Vec::new();
    let mut vertices_transparent = Vec::new();
    let mut indices_opaque = Vec::new();
    let mut indices_transparent = Vec::new();
    for core_pipeline_entity in query.iter() {
        let ExtractedCorePipelineEntity {
            mesh: mesh_handle,
            transform,
            color,
            image: image_handle,
        } = core_pipeline_entity;

        let transparent = images.get(image_handle).unwrap().translucent || color.a() < 1.0;
        let vertices = if transparent {
            &mut vertices_transparent
        } else {
            &mut vertices_opaque
        };
        let indices = if transparent {
            &mut indices_transparent
        } else {
            &mut indices_opaque
        };

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
    commands.spawn().insert(CorePipelineBatch {
        vertices: vertices_opaque,
        indices: indices_opaque,
        transparent: false,
    });
    commands.spawn().insert(CorePipelineBatch {
        vertices: vertices_transparent,
        indices: indices_transparent,
        transparent: true,
    });
}

pub fn prepare_bindless_textures(
    window: NonSend<Window>,
    images: Res<Assets<Image>>,
    mut texture_buffer: NonSendMut<TextureBuffer>,
    query: Query<(&Handle<Image>, With<ExtractedCorePipelineEntity>)>,
) {
    let display = window.display();
    for (image_handle, _) in query.iter() {
        let image = images.get(image_handle).unwrap();
        texture_buffer.insert(image_handle.clone_weak(), display, image);
    }
}
