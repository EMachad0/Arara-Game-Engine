use arara_app::{PluginGroup, PluginGroupBuilder};

pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(arara_time::TimePlugin);
        group.add(arara_diagnostic::DiagnosticsPlugin);
        group.add(arara_window::WindowPlugin);
        group.add(arara_render::RenderPlugin);
        group.add(arara_transform::TransformPlugin);
    }
}
