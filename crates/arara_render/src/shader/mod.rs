mod shader;
mod shader_loader;

use arara_app::{App, Plugin};
use arara_asset::{AddAsset, AssetServer, Assets, Handle, LoadState};
use arara_ecs::system::{Commands, NonSend, NonSendMut, Res};
use arara_utils::tracing::debug;
use arara_window::Window;
use glium::Program;
pub use shader::Shader;
pub use shader_loader::ShaderLoader;

/// Adds the [`Shader`] as an asset and makes sure that they are extracted and prepared for the GPU.
pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<ShaderLoader>()
            .add_asset::<Shader>();
    }
}

#[derive(Default)]
pub struct DefaultShader {
    pub vertex_shader: Handle<Shader>,
    pub fragment_shader: Handle<Shader>,
}

pub fn load_default_shader(mut commands: Commands, asset_server: Res<AssetServer>) {
    let vertex_shader = asset_server.load("shaders/vertex_shader_src.vert");
    let fragment_shader = asset_server.load("shaders/fragment_shader_src.frag");
    commands.insert_resource(DefaultShader {
        vertex_shader,
        fragment_shader,
    });
}

#[derive(Default)]
pub struct DefaultShaderProgram {
    pub program: Option<Program>,
}

pub(crate) fn init_default_shader_program(
    asset_server: Res<AssetServer>,
    shaders: Res<Assets<Shader>>,
    window: NonSend<Window>,
    default_shader: Res<DefaultShader>,
    mut default_shader_program: NonSendMut<DefaultShaderProgram>,
) {
    if default_shader_program.program.is_some() {
        return;
    }

    let display = window.display();
    let DefaultShader {
        vertex_shader,
        fragment_shader,
    } = &*default_shader;

    let load_state = asset_server.get_group_load_state([vertex_shader.id, fragment_shader.id]);
    if load_state == LoadState::Loading || load_state == LoadState::NotLoaded {
        debug!("loading");
        return;
    }

    let vertex_shader = shaders.get(vertex_shader).unwrap();
    let fragment_shader = shaders.get(fragment_shader).unwrap();
    default_shader_program.program = Some(
        Program::from_source(
            display,
            vertex_shader.source(),
            fragment_shader.source(),
            None,
        )
        .unwrap(),
    );
}
