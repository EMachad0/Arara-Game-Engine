mod input;
pub mod keyboard;
pub mod mouse;
pub mod system;

use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemLabel},
    system::IntoSystem,
};
use arara_app::prelude::*;
pub use input::*;
use keyboard::{keyboard_input_system, KeyCode, KeyboardInput};
use mouse::{mouse_button_input_system, MouseButton, MouseButtonInput, MouseMotion, MouseWheel};

pub mod prelude {
    pub use crate::{
        keyboard::KeyCode,
        mouse::MouseButton,
        Input,
    };
}


/// Adds keyboard and mouse input to an App
#[derive(Default)]
pub struct InputPlugin;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct InputSystem;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // keyboard
            .add_event::<KeyboardInput>()
            .init_resource::<Input<KeyCode>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                keyboard_input_system.system().label(InputSystem),
            )
            // mouse
            .add_event::<MouseButtonInput>()
            .add_event::<MouseMotion>()
            .add_event::<MouseWheel>()
            .init_resource::<Input<MouseButton>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                mouse_button_input_system.system().label(InputSystem),
            );
    }
}

/// The current "press" state of an element
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub enum ElementState {
    Pressed,
    Released,
}

impl ElementState {
    pub fn is_pressed(&self) -> bool {
        matches!(self, ElementState::Pressed)
    }
}
