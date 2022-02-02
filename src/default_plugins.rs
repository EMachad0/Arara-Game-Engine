use arara_app::{PluginGroup, PluginGroupBuilder};

pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(arara_logger::LoggerPlugin);
        group.add(arara_time::TimePlugin);
        group.add(arara_diagnostic::DiagnosticsPlugin);
        group.add(arara_input::InputPlugin);
        group.add(arara_asset::AssetPlugin);
        group.add(arara_window::WindowPlugin);
        group.add(arara_render::RenderPlugin);
        group.add(arara_core_pipeline::CorePipelinePlugin);
        group.add(arara_sprite::SpritePlugin);
        group.add(arara_transform::TransformPlugin);
        group.add(arara_camera::CameraPlugin);
        group.add(arara_camera::FlyCameraPlugin);
    }
}
