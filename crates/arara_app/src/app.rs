use crate::{AppExit, CoreStage, Plugin, PluginGroup, PluginGroupBuilder, StartupStage};
use arara_ecs::{
    event::Events,
    prelude::FromWorld,
    schedule::{IntoSystemDescriptor, RunOnce, Schedule, Stage, StageLabel, SystemStage},
    system::Resource,
    world::World,
};
use arara_utils::tracing::debug;

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
    pub fn new() -> Self {
        let mut app = Self::default();
        app.add_core_stages().add_event::<AppExit>();
        app
    }

    pub fn update(&mut self) {
        #[cfg(feature = "trace")]
        let arara_frame_update_span = info_span!("frame");
        #[cfg(feature = "trace")]
        let _arara_frame_update_guard = arara_frame_update_span.enter();

        self.schedule.run(&mut self.world);
    }

    pub fn run(&mut self) {
        #[cfg(feature = "trace")]
        let arara_app_run_span = info_span!("arara_app");
        #[cfg(feature = "trace")]
        let _arara_app_run_guard = arara_app_run_span.enter();

        self.debug_stage_order();
        let mut app = std::mem::replace(self, App::default());
        let runnable = std::mem::replace(&mut app.runnable, Box::new(run_once));
        (runnable)(app);
    }

    pub fn debug_stage_order(&self) {
        debug!("------------ Stages ------------");
        for stage in self.schedule.iter_stages() {
            debug!("{:?}", stage.0);
        }
    }

    pub fn set_runnable<T>(&mut self, runnable: T) -> &mut Self
    where
        T: 'static + Fn(App),
    {
        self.runnable = Box::new(runnable);
        self
    }

    pub fn init_resource<T>(&mut self) -> &mut Self
    where
        T: FromWorld + 'static + Send + Sync,
    {
        if !self.world.contains_resource::<T>() {
            let resource = T::from_world(&mut self.world);
            self.insert_resource(resource);
        }
        self
    }

    pub fn init_non_send_resource<R>(&mut self) -> &mut Self
    where
        R: FromWorld + 'static,
    {
        if self.world.get_non_send_resource::<R>().is_none() {
            let resource = R::from_world(&mut self.world);
            self.world.insert_non_send(resource);
        }
        self
    }

    pub fn insert_resource<T>(&mut self, resource: T) -> &mut Self
    where
        T: 'static + Send + Sync,
    {
        self.world.insert_resource(resource);
        self
    }

    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        debug!("added plugin: {}", plugin.name());
        plugin.build(self);
        self
    }

    pub fn add_plugins<T: PluginGroup>(&mut self, mut group: T) -> &mut Self {
        let mut plugin_group_builder = PluginGroupBuilder::default();
        group.build(&mut plugin_group_builder);
        plugin_group_builder.finish(self);
        self
    }

    pub fn add_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        self.add_system_to_stage(CoreStage::Update, system)
    }

    pub fn add_system_to_stage<Params>(
        &mut self,
        stage_label: impl StageLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        self.schedule.add_system_to_stage(stage_label, system);
        self
    }

    pub fn add_startup_system<Params>(
        &mut self,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        self.add_startup_system_to_stage(StartupStage::Startup, system)
    }

    pub fn add_startup_system_to_stage<Params>(
        &mut self,
        stage_label: impl StageLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        self.schedule
            .stage(CoreStage::Startup, |schedule: &mut Schedule| {
                schedule.add_system_to_stage(stage_label, system)
            });
        self
    }

    pub fn add_stage(&mut self, stage_label: impl StageLabel, stage: impl Stage) -> &mut Self {
        self.schedule.add_stage(stage_label, stage);
        self
    }

    pub fn add_stage_after<S: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: S,
    ) -> &mut Self {
        self.schedule.add_stage_after(target, label, stage);
        self
    }

    pub fn add_stage_before<S: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: S,
    ) -> &mut Self {
        self.schedule.add_stage_before(target, label, stage);
        self
    }

    fn add_core_stages(&mut self) -> &mut Self {
        self.add_stage(CoreStage::First, SystemStage::parallel())
            .add_stage(CoreStage::EventUpdateStage, SystemStage::parallel())
            .add_stage(
                CoreStage::Startup,
                Schedule::default()
                    .with_run_criteria(RunOnce::default())
                    .with_stage(StartupStage::PreStartup, SystemStage::parallel())
                    .with_stage(StartupStage::Startup, SystemStage::parallel())
                    .with_stage(StartupStage::PostStartup, SystemStage::parallel()),
            )
            .add_stage(CoreStage::PreUpdate, SystemStage::parallel())
            .add_stage(CoreStage::Update, SystemStage::parallel())
            .add_stage(CoreStage::PostUpdate, SystemStage::parallel())
    }

    pub fn add_event<T>(&mut self) -> &mut Self
    where
        T: Resource,
    {
        self.init_resource::<Events<T>>()
            .add_system_to_stage(CoreStage::First, Events::<T>::update_system)
    }
}

fn run_once(mut app: App) {
    app.update();
}
