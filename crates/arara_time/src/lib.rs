mod time;
mod stopwatch;
mod timer;

pub use time::*;
pub use stopwatch::*;
pub use timer::*;

pub mod prelude {
    pub use crate::{
        time::Time,
        stopwatch::Stopwatch,
        timer::Timer,
    };
}

use bevy_ecs::prelude::*;
use arara_app::{AppBuilder, CoreStage, Plugin};

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub enum CoreSystem {
    /// Updates the elapsed time. Any system that interacts with [Time] component should run after this.
    Time,
}

#[derive(Default)]
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Time>()
            .add_system_to_stage(
                CoreStage::First,
                update_time.exclusive_system().label(CoreSystem::Time),
            );
    }
}