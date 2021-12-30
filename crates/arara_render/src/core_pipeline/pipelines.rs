use arara_asset::{AssetServer, Handle};
use arara_ecs::world::{FromWorld, World};

use crate::{RenderPipelineDescriptor, Shader, SpecializedPipeline};

#[derive(Debug, Clone)]
pub struct DefaultShader {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
}

impl FromWorld for DefaultShader {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let vertex_shader = asset_server.load("shaders/vertex_shader_src.vert");
        let fragment_shader = asset_server.load("shaders/fragment_shader_src.frag");
        Self {
            vertex_shader,
            fragment_shader,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpaquePipelineKey;

pub struct OpaquePipeline {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
}

impl FromWorld for OpaquePipeline {
    fn from_world(world: &mut World) -> Self {
        let default_shaders = world.get_resource::<DefaultShader>().unwrap();
        let DefaultShader {
            vertex_shader,
            fragment_shader,
        } = default_shaders.clone();
        Self {
            vertex_shader,
            fragment_shader,
        }
    }
}

impl SpecializedPipeline for OpaquePipeline {
    type Key = OpaquePipelineKey;

    fn specialize(&self, _key: Self::Key) -> RenderPipelineDescriptor {
        let OpaquePipeline {
            vertex_shader,
            fragment_shader,
        } = self;
        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };
        RenderPipelineDescriptor {
            vertex_shader: vertex_shader.clone_weak(),
            fragment_shader: fragment_shader.clone_weak(),
            draw_parameters,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TransparentPipelineKey;

pub struct TransparentPipeline {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
}

impl FromWorld for TransparentPipeline {
    fn from_world(world: &mut World) -> Self {
        let default_shaders = world.get_resource::<DefaultShader>().unwrap();
        let DefaultShader {
            vertex_shader,
            fragment_shader,
        } = default_shaders.clone();
        Self {
            vertex_shader,
            fragment_shader,
        }
    }
}

impl SpecializedPipeline for TransparentPipeline {
    type Key = TransparentPipelineKey;

    fn specialize(&self, _key: Self::Key) -> RenderPipelineDescriptor {
        let TransparentPipeline {
            vertex_shader,
            fragment_shader,
        } = self;
        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                ..Default::default()
            },
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
