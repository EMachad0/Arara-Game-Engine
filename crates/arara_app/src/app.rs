use bevy_ecs::{schedule::{Schedule, Stage}, world::World};

use arara_logger::*;

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
        trace!("App updated!");
        self.schedule.run(&mut self.world);
    }

    pub fn run(mut self) {
        let runnable = std::mem::replace(&mut self.runnable, Box::new(run_once));
        (runnable)(self);
    }
}

fn run_once(mut app: App) {
    app.update();
}