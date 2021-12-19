use arara_ecs::{
    schedule::{Schedule, Stage},
    world::World,
};
use arara_utils::tracing::debug;

use crate::app_builder::AppBuilder;

#[cfg(feature = "trace")]
use arara_utils::tracing::info_span;

pub struct App {
    pub world: World,
    pub schedule: Schedule,
    pub runnable: Box<dyn Fn(App)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: Default::default(),
            schedule: Schedule::default(),
            runnable: Box::new(run_once),
        }
    }
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn update(&mut self) {
        #[cfg(feature = "trace")]
        let bevy_frame_update_span = info_span!("frame");
        #[cfg(feature = "trace")]
        let _bevy_frame_update_guard = bevy_frame_update_span.enter();

        self.schedule.run(&mut self.world);
    }

    pub fn run(mut self) {
        #[cfg(feature = "trace")]
        let bevy_app_run_span = info_span!("arara_app");
        #[cfg(feature = "trace")]
        let _bevy_app_run_guard = bevy_app_run_span.enter();

        self.debug_stage_order();
        let runnable = std::mem::replace(&mut self.runnable, Box::new(run_once));
        (runnable)(self);
    }

    pub fn debug_stage_order(&self) {
        debug!("------------ Stages ------------");
        for stage in self.schedule.iter_stages() {
            debug!("{:?}", stage.0);
        }
    }
}

fn run_once(mut app: App) {
    app.update();
}
