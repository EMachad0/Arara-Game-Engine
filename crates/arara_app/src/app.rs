use bevy_ecs::world::World;

use crate::app_builder::AppBuilder;

pub struct App {
    pub world: World,
    pub runnable: Box<dyn Fn(App)>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: Default::default(),
            runnable: Box::new(run_once),        
        }
    }
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn update(&mut self) {
        // TODO
        println!("App updated!")
    }

    pub fn run(mut self) {
        let runnable = std::mem::replace(&mut self.runnable, Box::new(run_once));
        (runnable)(self);
    }
}

fn run_once(mut app: App) {
    app.update();
}