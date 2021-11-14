use bevy_ecs::{
    world::{World, FromWorld},
};

use arara_logger::*;

use crate::app::App;
use crate::plugin::Plugin;

pub struct AppBuilder {
    pub app : App,
}

impl Default for AppBuilder {
    fn default() -> Self {
        let app_builder = Self {
            app: App::default(),
        };
        app_builder
    }
}

impl AppBuilder {
    pub fn build(&mut self) -> App {
        std::mem::take(&mut self.app)
    }

    pub fn set_runnable<T>(&mut self, runnable: T) -> &mut Self
    where 
        T: 'static + Fn(App)
    {
        self.app.runnable = Box::new(runnable);
        self
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.app.world
    }

    pub fn init_resource<T>(&mut self) -> &mut Self
    where
        T: FromWorld + 'static + Send + Sync,
    {
        if !self.world_mut().contains_resource::<T>() {
            let resource = T::from_world(self.world_mut());
            self.insert_resource(resource);
        }
        self
    }

    pub fn init_non_send_resource<R>(&mut self) -> &mut Self
    where
        R: FromWorld + 'static,
    {
        if self.app.world.get_non_send_resource::<R>().is_none() {
            let resource = R::from_world(self.world_mut());
            self.app.world.insert_non_send(resource);
        }
        self
    }

    pub fn insert_resource<T>(&mut self, resource: T) -> &mut Self
    where
        T: 'static + Send + Sync,
    {
        self.app.world.insert_resource(resource);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        debug!("added plugin: {}", plugin.name());
        plugin.build(self);
        self
    }
}