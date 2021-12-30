use std::fmt;

use arara_asset::Handle;
use bevy_reflect::TypeUuid;
use glium::{DrawParameters, Program};

use crate::Shader;

#[derive(TypeUuid)]
#[uuid = "9c0a3d57-c651-431e-9e9c-198429eb95f5"]
pub struct RenderPipeline {
    pub program: Program,
    pub parameters: DrawParameters<'static>,
}

impl fmt::Debug for RenderPipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderPipeline")
            .field("parameters", &self.parameters)
            .finish()
    }
}

/// Describes a render (graphics) pipeline.
#[derive(Clone, Debug)]
pub struct RenderPipelineDescriptor {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
    pub draw_parameters: DrawParameters<'static>,
}
