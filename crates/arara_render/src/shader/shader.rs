use glium::program::ShaderStage;
use std::borrow::Cow;

use bevy_reflect::TypeUuid;

/// A shader, as defined by its [ShaderSource] and [ShaderStage]
/// This is an "unprocessed" shader. It can contain preprocessor directives.
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "d95bc916-6c55-4de3-9622-37e7b6969fda"]
pub struct Shader {
    source: Source,
}

impl Shader {
    pub fn from_glsl(source: impl Into<Cow<'static, str>>, stage: ShaderStage) -> Shader {
        let source = source.into();
        Shader {
            source: Source::Glsl(source, stage),
        }
    }

    pub fn source(&self) -> &str {
        match &self.source {
            Source::Glsl(src, _) => &src,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Source {
    Glsl(Cow<'static, str>, ShaderStage),
    // Wgsl(Cow<'static, str>),
    // SpirV(Cow<'static, [u8]>),
}
