use bevy_ecs::{
    schedule::{Schedule, Stage},
    world::World,
};

use crate::app_builder::AppBuilder;

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
        self.schedule.run(&mut self.world);
    }

    pub fn run(mut self) {
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
