mod bundle;

use arara_app::{App, Plugin};

#[derive(Default)]
pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, _app: &mut App) {}
}
