use arara_asset::{AssetServer, Handle};
use arara_ecs::world::{FromWorld, World};
use arara_render::{RenderPipelineDescriptor, Shader, SpecializedPipeline};

pub struct SpritePipeline {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
}

impl FromWorld for SpritePipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let vertex_shader = asset_server.load("shaders/vertex_shader_sprite_src.vert");
        let fragment_shader = asset_server.load("shaders/fragment_shader_sprite_src.frag");
        Self {
            vertex_shader,
            fragment_shader,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SpritePipelineKey;

impl SpecializedPipeline for SpritePipeline {
    type Key = SpritePipelineKey;

    fn specialize(&self, _key: Self::Key) -> RenderPipelineDescriptor {
        let SpritePipeline {
            vertex_shader,
            fragment_shader,
        } = self;
        let draw_parameters = glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..Default::default()
        };
        RenderPipelineDescriptor {
            vertex_shader: vertex_shader.clone_weak(),
            fragment_shader: fragment_shader.clone_weak(),
            draw_parameters,
        }
    }
}
