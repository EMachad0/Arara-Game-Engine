use bevy_ecs::{schedule::{RunOnce, Schedule, Stage, StageLabel, SystemDescriptor, SystemStage}, world::{World, FromWorld}};

use arara_logger::*;

use crate::{CoreStage, StartupStage, app::App, plugin::Plugin};

pub struct AppBuilder {
    pub app : App,
}

impl Default for AppBuilder {
    fn default() -> Self {
        let mut app_builder = Self {
            app: App::default(),
        };
        app_builder.add_core_stages();
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

    pub fn add_system(&mut self, system: impl Into<SystemDescriptor>) -> &mut Self {
        self.add_system_to_stage(CoreStage::Update, system)
    }

    pub fn add_system_to_stage(
        &mut self,
        stage_label: impl StageLabel,
        system: impl Into<SystemDescriptor>
    ) -> &mut Self {
        self.app.schedule.add_system_to_stage(stage_label, system);
        self
    }

    pub fn add_startup_system(&mut self, system: impl Into<SystemDescriptor>) -> &mut Self {
        self.add_system_to_startup_stage(StartupStage::Startup, system)
    }

    pub fn add_system_to_startup_stage(
        &mut self,
        stage_label: impl StageLabel,
        system: impl Into<SystemDescriptor>
    ) -> &mut Self {
        self.app.schedule.stage(CoreStage::Startup, |schedule: &mut Schedule| {
            schedule.add_system_to_stage(stage_label, system)
        });
        self
    }

    pub fn add_core_stages(&mut self) -> &mut Self {
        self.add_stage(CoreStage::First, SystemStage::parallel())
            .add_stage(
                CoreStage::Startup,
                Schedule::default()
                    .with_run_criteria(RunOnce::default())
                    .with_stage(StartupStage::Startup, SystemStage::parallel()),
                )
            .add_stage(CoreStage::Update, SystemStage::parallel())
    }

    pub fn add_stage(&mut self, stage_label: impl StageLabel, stage: impl Stage) -> &mut Self {
        self.app.schedule.add_stage(stage_label, stage);
        self
    }
}