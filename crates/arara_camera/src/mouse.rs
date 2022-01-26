use arara_ecs::{
    prelude::Component,
    query::With,
    system::{NonSend, Query},
};
use arara_transform::Transform;
use arara_window::Window;
use glam::{vec2, Vec2, Vec4};

use crate::Camera;

#[derive(Component, Default, Debug)]
pub struct WorldMouse2d {
    pos: Option<Vec2>,
}

impl WorldMouse2d {
    pub fn pos(&self) -> Option<Vec2> {
        self.pos
    }
}

pub(crate) fn track_world_mouse_2d(
    // need to get window dimensions
    window: NonSend<Window>,
    // query to get camera transform
    mut query: Query<(&Transform, &mut WorldMouse2d), With<Camera>>,
) {
    // sometimes there is no camera
    if query.is_empty() {
        return;
    }

    // assuming there is exactly one main camera entity, so this is OK
    let (transform, mut mouse) = query.single_mut();

    // check if the cursor is in the primary window
    mouse.pos = window.cursor_position().map(|pos| {
        let pos = Vec2::from(pos);

        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // apply the camera transform
        let position = transform.compute_matrix() * Vec4::from((p, 0.0, 1.0));
        vec2(position.x, position.y)
    });
}
