use arara_ecs::world::World;
use arara_window::Window;
use glium::Surface;

use crate::{
    render_phase::{DrawFunctions, RenderPhase, TrackedFrame},
    ClearColor, Opaque3D,
};

pub(crate) fn draw_frame(world: &mut World) {
    let window = world.get_non_send_resource::<Window>().unwrap();
    let clear_color = world.get_resource::<ClearColor>().unwrap();
    let opaques = world.get_resource::<RenderPhase<Opaque3D>>().unwrap();
    let opaque_draw_functions = world.get_resource::<DrawFunctions<Opaque3D>>().unwrap();

    let display = window.display();

    let clear_color = (
        clear_color.0.r(),
        clear_color.0.g(),
        clear_color.0.b(),
        clear_color.0.a(),
    );

    let mut frame = display.draw();
    frame.clear_color_and_depth(clear_color, 1.0);

    let mut tracked_frame = TrackedFrame::new(frame);

    let mut draw_functions = opaque_draw_functions.write();
    for item in opaques.items.iter() {
        let draw_function = draw_functions.get_mut(item.draw_function).unwrap();
        draw_function.draw(world, &mut tracked_frame, item);
    }

    tracked_frame.frame.finish().unwrap();
}
