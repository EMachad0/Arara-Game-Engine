use arara_asset::{AssetEvent, Assets, Handle};
use arara_ecs::{
    event::EventReader,
    system::{NonSend, NonSendMut, Res},
};
use arara_utils::{
    tracing::{error, trace},
    HashMap, HashSet,
};
use arara_window::Window;
use glium::Display;
use thiserror::Error;

use crate::{
    render_resource::{RenderPipeline, RenderPipelineDescriptor},
    Shader,
};

#[derive(Default)]
struct ShaderCache {
    data: HashMap<Handle<Shader>, ShaderMetaData>,
    shaders: HashMap<Handle<Shader>, Shader>,
}

#[derive(Default)]
pub struct ShaderMetaData {
    pipelines: HashSet<CachedPipelineId>,
}

impl ShaderCache {
    fn get(&mut self, handle: &Handle<Shader>) -> Result<Shader, RenderPipelineError> {
        let shader = self
            .shaders
            .get(handle)
            .ok_or_else(|| RenderPipelineError::ShaderNotLoaded(handle.clone_weak()))?;
        Ok(shader.clone())
    }

    fn get_dependent_pipelines(&mut self, handle: &Handle<Shader>) -> Vec<CachedPipelineId> {
        match self.data.get_mut(&handle) {
            Some(data) => data.pipelines.iter().cloned().collect(),
            None => Vec::new(),
        }
    }

    fn set_shader(&mut self, handle: &Handle<Shader>, shader: Shader) -> Vec<CachedPipelineId> {
        let pipelines_to_queue = self.get_dependent_pipelines(handle);
        self.shaders.insert(handle.clone_weak(), shader);
        pipelines_to_queue
    }

    fn remove(&mut self, handle: &Handle<Shader>) -> Vec<CachedPipelineId> {
        let pipelines_to_queue = self.get_dependent_pipelines(handle);
        self.shaders.remove(handle);
        pipelines_to_queue
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct CachedPipelineId(usize);

// TODO: make arara::Window hava an Arc<Display>
#[derive(Default)]
pub struct RenderPipelineCache {
    // display: Display,
    // layout_cache: LayoutCache, // Buffers
    shader_cache: ShaderCache,
    pipelines: Vec<CachedPipeline>,
    waiting_pipelines: HashSet<CachedPipelineId>,
}

impl RenderPipelineCache {
    #[inline]
    pub fn get_state(&self, id: CachedPipelineId) -> &CachedPipelineState {
        &self.pipelines[id.0].state
    }

    #[inline]
    pub fn get(&self, id: CachedPipelineId) -> Option<&RenderPipeline> {
        if let CachedPipelineState::Ok(pipeline) = &self.pipelines[id.0].state {
            Some(pipeline)
        } else {
            None
        }
    }

    pub fn queue(&mut self, descriptor: RenderPipelineDescriptor) -> CachedPipelineId {
        let id = CachedPipelineId(self.pipelines.len());
        self.pipelines.push(CachedPipeline {
            descriptor,
            state: CachedPipelineState::Queued,
        });
        self.waiting_pipelines.insert(id);
        id
    }

    fn set_shader(&mut self, handle: &Handle<Shader>, shader: &Shader) {
        let pipelines_to_queue = self.shader_cache.set_shader(handle, shader.clone());
        for cached_pipeline in pipelines_to_queue {
            self.pipelines[cached_pipeline.0].state = CachedPipelineState::Queued;
            self.waiting_pipelines.insert(cached_pipeline);
        }
    }

    fn remove_shader(&mut self, shader: &Handle<Shader>) {
        let pipelines_to_queue = self.shader_cache.remove(shader);
        for cached_pipeline in pipelines_to_queue {
            self.pipelines[cached_pipeline.0].state = CachedPipelineState::Queued;
            self.waiting_pipelines.insert(cached_pipeline);
        }
    }

    pub fn process_queue(&mut self, display: &Display) {
        let pipelines = std::mem::take(&mut self.waiting_pipelines);
        for id in pipelines {
            // debug!("trying id {:?}", id);
            let state = &mut self.pipelines[id.0];
            match &state.state {
                CachedPipelineState::Ok(_) => continue,
                CachedPipelineState::Queued => {}
                CachedPipelineState::Err(err) => {
                    match err {
                        RenderPipelineError::ShaderNotLoaded(_) => { /* retry */ }
                        // shader could not be processed ... retrying won't help
                        RenderPipelineError::ProgramCreationError(err) => {
                            error!("failed to process shaders: {}", err);
                            continue;
                        }
                    }
                }
            }

            let descriptor = &state.descriptor;
            let vertex_shader = match self.shader_cache.get(&descriptor.vertex_shader) {
                Ok(shader) => shader,
                Err(err) => {
                    state.state = CachedPipelineState::Err(err);
                    self.waiting_pipelines.insert(id);
                    continue;
                }
            };

            let fragment_shader = match self.shader_cache.get(&descriptor.fragment_shader) {
                Ok(shader) => shader,
                Err(err) => {
                    state.state = CachedPipelineState::Err(err);
                    self.waiting_pipelines.insert(id);
                    continue;
                }
            };

            let parameters = descriptor.draw_parameters.clone();
            let program = match glium::Program::from_source(
                display,
                vertex_shader.source(),
                fragment_shader.source(),
                None,
            ) {
                Ok(program) => program,
                Err(err) => {
                    state.state =
                        CachedPipelineState::Err(RenderPipelineError::ProgramCreationError(err));
                    self.waiting_pipelines.insert(id);
                    continue;
                }
            };

            let pipeline = RenderPipeline {
                program,
                parameters,
            };

            state.state = CachedPipelineState::Ok(pipeline);
        }
    }
}

pub(crate) fn process_pipeline_queue(
    mut cache: NonSendMut<RenderPipelineCache>,
    window: NonSend<Window>,
) {
    cache.process_queue(window.display());
}

pub(crate) fn extract_shaders(
    shaders: Res<Assets<Shader>>,
    mut events: EventReader<AssetEvent<Shader>>,
    mut cache: NonSendMut<RenderPipelineCache>,
) {
    for event in events.iter() {
        trace!("recieved event {:?}", event);
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                if let Some(shader) = shaders.get(handle) {
                    cache.set_shader(handle, shader);
                }
            }
            AssetEvent::Removed { handle } => cache.remove_shader(handle),
        }
    }
}

struct CachedPipeline {
    descriptor: RenderPipelineDescriptor,
    state: CachedPipelineState,
}

#[derive(Debug)]
pub enum CachedPipelineState {
    Queued,
    Ok(RenderPipeline),
    Err(RenderPipelineError),
}

impl CachedPipelineState {
    pub fn unwrap(&self) -> &RenderPipeline {
        match self {
            CachedPipelineState::Ok(pipeline) => pipeline,
            CachedPipelineState::Queued => {
                panic!("Pipeline has not been compiled yet. It is still in the 'Queued' state.")
            }
            CachedPipelineState::Err(err) => panic!("{}", err),
        }
    }
}

#[derive(Error, Debug)]
pub enum RenderPipelineError {
    #[error(
        "Pipeline cound not be compiled because the following shader is not loaded yet: {0:?}"
    )]
    ShaderNotLoaded(Handle<Shader>),
    #[error(transparent)]
    ProgramCreationError(#[from] glium::ProgramCreationError),
}
