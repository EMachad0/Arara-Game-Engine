mod default_plugins;
pub use default_plugins::*;

pub mod prelude;

pub mod camera {
    pub use arara_camera::*;
}

pub mod geometry {
    pub use arara_geometry::*;
}

pub mod shaders {
    pub use arara_shaders::*;
}

pub mod transform {
    pub use arara_transform::*;
}

pub mod logger {
    pub use arara_logger::*;
}

pub mod window {
    pub use arara_window::*;
}

pub mod app {
    pub use arara_app::*;
}

pub mod ecs {
    pub use bevy_ecs::*;    
}

pub mod utils {
    pub use arara_utils::*;
}

pub mod time {
    pub use arara_time::*;
}

pub mod diagnostics {
    pub use arara_diagnostic::*;
}

pub mod render {
    pub use arara_render::*;
}

pub mod math {
    pub use glam::*;
    pub use std::f32::consts::*;
    pub fn rad(deg: f32) -> f32 {
        return deg * PI / 180.0;
    }
}

pub mod input {
    pub use arara_input::*;
}

pub mod asset {
    pub use arara_asset::*;
}