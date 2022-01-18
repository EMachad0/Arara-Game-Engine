use arara_ecs::world::World;
use arara_window::Window;

use crate::{ClearColor, RenderPhases, TrackedFrame};

pub(crate) fn draw_frame(world: &mut World) {
    let window = world.get_non_send_resource::<Window>().unwrap();
    let clear_color = world.get_resource::<ClearColor>().unwrap();
    let phases = world.get_resource::<RenderPhases>().unwrap();

    let display = window.display();

    let mut tracked_frame = TrackedFrame::new(display);
    tracked_frame.clear_color_and_depth(clear_color.0);

    phases.run(world, &mut tracked_frame);

    tracked_frame.finish().unwrap();
}
