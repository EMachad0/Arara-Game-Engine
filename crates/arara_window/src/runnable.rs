use bevy_ecs::prelude::World;

use glium::{
    self,
    glutin::{
        self,
        event::*,
        window::WindowId
    }
};

use arara_logger::*;
use arara_app::App;

use crate::{EventLoop, Window};

pub fn run(mut app: App) {
    let mut mouse_pressed = false;

    let mut ev = app.world.get_non_send_resource_mut::<EventLoop>().unwrap();
    let event_loop = ev.take();

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::MainEventsCleared => {
                app.update();
            }
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    if mouse_pressed {
                        // camera_controller.process_mouse(delta.0, delta.1);
                    }
                }
                _ => (),
            },
            Event::WindowEvent { event, window_id } => {
                
                if window_id != get_primaty_window_id(&app.world) { 
                    trace!("recieved event for unknown window_id");
                    return;
                }

                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state,
                                ..
                            },
                        ..
                    } => {
                        // camera_controller.process_keyboard(key, state);
                    }
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    } => {
                        mouse_pressed = state == ElementState::Pressed;
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        // camera_controller.process_scroll(&delta);
                    }
                    WindowEvent::Resized(physical_size) => {
                        // perspective.resize_from_size(physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // perspective.resize_from_size(*new_inner_size);
                    }
                    _ => return,
                }
            }
            _ => (),
        }

        let dt = std::time::Duration::from_nanos(16_666_667);
        let next_frame_time = std::time::Instant::now() + dt;
        // camera_controller.update_camera(&mut camera, dt);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}

fn get_primaty_window_id(world: &World) -> WindowId {
    let window = world.get_non_send_resource::<Window>().unwrap();
    window.display().gl_window().window().id()
}
