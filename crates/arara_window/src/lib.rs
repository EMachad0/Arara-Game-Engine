mod converters;
mod event;
mod event_loop;
mod runnable;
mod window;
mod window_props;

pub use event::*;
pub use event_loop::*;
pub use runnable::*;
pub use window::*;
pub use window_props::*;

pub mod prelude {
    pub use crate::{event::*, window::Window, window_props::WindowProps, WindowPlugin};
}

use arara_app::{App, AppExit, Plugin};
use arara_ecs::event::{EventReader, EventWriter};

#[derive(Default)]
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app_builder: &mut App) {
        app_builder
            .add_event::<WindowResized>()
            .add_event::<WindowCreated>()
            .add_event::<WindowCloseRequested>()
            .add_event::<CloseWindow>()
            .add_event::<CursorMoved>()
            .add_event::<CursorEntered>()
            .add_event::<CursorLeft>()
            .add_event::<ReceivedCharacter>()
            .add_event::<WindowFocused>()
            .add_event::<FileDragAndDrop>()
            .add_event::<WindowMoved>()
            .init_non_send_resource::<EventLoop>()
            .init_non_send_resource::<Window>()
            .add_system(exit_on_window_close)
            .set_runnable(run);
    }
}

pub fn exit_on_window_close(
    mut app_exit_events: EventWriter<AppExit>,
    mut window_close_requested_events: EventReader<WindowCloseRequested>,
) {
    if window_close_requested_events.iter().next().is_some() {
        app_exit_events.send(AppExit);
    }
}
