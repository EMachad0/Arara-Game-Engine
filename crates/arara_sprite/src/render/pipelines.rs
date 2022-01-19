use arara_asset::{AssetServer, Handle};
use arara_ecs::world::{FromWorld, World};
use arara_render::{RenderPipelineDescriptor, Shader, SpecializedPipeline};

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
pub struct CorePipelineKey {
    pub transparent: bool,
}

pub struct CorePipeline {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
}

impl FromWorld for CorePipeline {
    fn from_world(world: &mut World) -> Self {
        let default_shaders = match world.get_resource::<DefaultShader>() {
            Some(shaders) => shaders.clone(),
            None => DefaultShader::from_world(world),
        };
        let DefaultShader {
            vertex_shader,
            fragment_shader,
        } = default_shaders;
        Self {
            vertex_shader,
            fragment_shader,
        }
    }
}

impl SpecializedPipeline for CorePipeline {
    type Key = CorePipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let CorePipeline {
            vertex_shader,
            fragment_shader,
        } = self;
        let draw_parameters = if key.transparent {
            glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    ..Default::default()
                },
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                blend: glium::draw_parameters::Blend::alpha_blending(),
                ..Default::default()
            }
        } else {
            glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                ..Default::default()
            }
        };
        RenderPipelineDescriptor {
            vertex_shader: vertex_shader.clone_weak(),
            fragment_shader: fragment_shader.clone_weak(),
            draw_parameters,
        }
    }
}
