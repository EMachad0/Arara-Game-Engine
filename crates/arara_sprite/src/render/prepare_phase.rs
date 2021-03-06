use arara_asset::Assets;
use arara_ecs::prelude::*;
use arara_render::{Mesh, TextureBuffer};
use glam::vec4;

use crate::{render::extract_phase::ExtractedSprites, sprite::ExtractedSprite, QUAD_MESH_HANDLE};

#[derive(Copy, Clone)]
pub struct Vertex {
    i_position: [f32; 3],
    i_color: [f32; 4],
    i_tex_coord: [f32; 2],
    i_tex_id: u32,
}

glium::implement_vertex!(Vertex, i_position, i_color, i_tex_coord, i_tex_id);

#[derive(Component)]
pub struct SpriteBatch {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub(crate) fn prepare_sprite_phase(
    mut commands: Commands,
    mut extracts: ResMut<ExtractedSprites>,
    meshes: Res<Assets<Mesh>>,
    mut texture_buffer: NonSendMut<TextureBuffer>,
) {
    if extracts.items.is_empty() {
        return;
    }
    let mesh = meshes.get(QUAD_MESH_HANDLE).unwrap();
    let mut vertices = Vec::with_capacity(extracts.items.len() * 4);
    let mut indices = Vec::with_capacity(extracts.items.len() * 6);

    extracts
        .items
        .sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());

    for ExtractedSprite {
        image_handle,
        transform,
        color,
        uv_coord,
        z: _,
    } in extracts.items.iter()
    {
        let tex_id = texture_buffer.get_or_insert(image_handle.clone_weak());
        let color: [f32; 4] = color.to_owned().into();
        let offset = vertices.len() as u32;

        for vertex in mesh.vertices.iter() {
            let position = vec4(
                vertex.position[0],
                vertex.position[1],
                vertex.position[2],
                1.0,
            );
            let i_tex_coord = match uv_coord {
                Some(coord) => [
                    coord.point.x + vertex.tex_coord[0] * coord.size.x,
                    coord.point.y + vertex.tex_coord[1] * coord.size.y,
                ],
                None => vertex.tex_coord,
            };
            let position = *transform * position;
            vertices.push(Vertex {
                i_position: [position.x, position.y, position.z],
                i_color: color,
                i_tex_coord,
                i_tex_id: tex_id as u32,
            });
        }
        for idx in mesh.indices.iter() {
            indices.push(*idx + offset);
        }
    }
    commands.spawn().insert(SpriteBatch { vertices, indices });
}
