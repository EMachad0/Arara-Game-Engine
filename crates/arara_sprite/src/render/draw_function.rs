use arara_ecs::{
    system::{
        lifetimeless::{Read, SQuery, SRes},
        NonSend, SystemState,
    },
    world::World,
};
use arara_render::{
    CachedPipelinePhaseItem, Draw, EntityPhaseItem, ExtractedView, RenderPipelineCache,
    TextureBuffer, TrackedFrame,
};
use arara_window::Window;
use glium::implement_uniform_block;

use crate::render::{phase_items::Transparent2D, prepare_phase::SpriteBatch};

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

pub struct DrawSprite {
    params: SystemState<(
        NonSend<'static, Window>,
        NonSend<'static, TextureBuffer>,
        NonSend<'static, RenderPipelineCache>,
        SRes<ExtractedView>,
        SQuery<Read<SpriteBatch>>,
    )>,
}

impl DrawSprite {
    pub fn new(world: &mut World) -> Self {
        Self {
            params: SystemState::new(world),
        }
    }
}

impl Draw<Transparent2D> for DrawSprite {
    fn draw<'w>(&mut self, world: &'w World, frame: &mut TrackedFrame, item: &Transparent2D) {
        let (window, texture_buffer, pipeline_cache, view, query) = self.params.get(world);

        let display = window.display();

        let pv_matrix: [[f32; 4]; 4] = view.pv_matrix.to_cols_array_2d();
        let camera_uniform_buffer =
            glium::uniforms::UniformBuffer::new(display, CameraUniformBuffer::new(pv_matrix))
                .unwrap();

        let texture_uniform_buffer =
            glium::uniforms::UniformBuffer::new(display, texture_buffer.texture_uniform_buffer())
                .unwrap();

        let uniforms = glium::uniform! {
            camera: &camera_uniform_buffer,
            samplers: &texture_uniform_buffer,
        };

        let pipeline = match pipeline_cache.get(item.cached_pipeline()) {
            Some(pipeline) => pipeline,
            None => return,
        };

        let SpriteBatch { vertices, indices } = query.get(item.entity()).unwrap();

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer: glium::IndexBuffer<u32> =
            glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, indices)
                .unwrap();

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
}
