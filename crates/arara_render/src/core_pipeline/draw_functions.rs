use arara_ecs::{
    system::{
        lifetimeless::{Read, SQuery, SRes},
        NonSend, SystemState,
    },
    world::World,
};
use arara_window::Window;
use glam::Vec4;
use glium::implement_uniform_block;

use crate::{
    core_pipeline::prepare_phase::CorePipelineBatch, render_phase::Draw, BPLight,
    CachedPipelinePhaseItem, EntityPhaseItem, ExtractedView, PhaseItem,
    RenderPipelineCache, TextureBuffer, TrackedFrame,
};

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
    pub u_camera_pos: [f32; 4],
    pub u_light_pos: [f32; 4],
}

implement_uniform_block!(BPLightUniformBuffer, u_camera_pos, u_light_pos);

pub struct DrawSimpleMesh {
    params: SystemState<(
        NonSend<'static, Window>,
        NonSend<'static, TextureBuffer>,
        NonSend<'static, RenderPipelineCache>,
        SRes<BPLight>,
        SRes<ExtractedView>,
        SQuery<Read<CorePipelineBatch>>,
    )>,
}

impl DrawSimpleMesh {
    pub fn new(world: &mut World) -> Self {
        Self {
            params: SystemState::new(world),
        }
    }
}

impl<I: PhaseItem + CachedPipelinePhaseItem + EntityPhaseItem> Draw<I> for DrawSimpleMesh {
    fn draw<'w>(&mut self, world: &'w World, frame: &mut TrackedFrame, item: &I) {
        let (window, texture_buffer, pipeline_cache, bp_light, view, query) =
            self.params.get(world);

        let display = window.display();

        let pv_matrix: [[f32; 4]; 4] = view.pv_matrix.to_cols_array_2d();
        let camera_uniform_buffer =
            glium::uniforms::UniformBuffer::new(display, CameraUniformBuffer::new(pv_matrix))
                .unwrap();

        let texture_uniform_buffer =
            glium::uniforms::UniformBuffer::new(display, texture_buffer.texture_uniform_buffer())
                .unwrap();

        let bplight_uniform_buffer = glium::uniforms::UniformBuffer::new(
            display,
            BPLightUniformBuffer {
                u_camera_pos: Vec4::from((view.position, 0.0)).into(),
                u_light_pos: Vec4::from((bp_light.position, 0.0)).into(),
            },
        )
        .unwrap();

        let uniforms = glium::uniform! {
            camera: &camera_uniform_buffer,
            bplight: &bplight_uniform_buffer,
            samplers: &texture_uniform_buffer,
        };

        let pipeline = match pipeline_cache.get(item.cached_pipeline()) {
            Some(pipeline) => pipeline,
            None => return,
        };

        let CorePipelineBatch {
            vertices,
            indices,
            transparent: _,
        } = query.get(item.entity()).unwrap();

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
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
