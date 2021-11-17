use bevy_ecs::prelude::World;

use glium::{self, glutin::{event::*, event_loop::ControlFlow, window::WindowId}};

use arara_time::Time;
use arara_app::App;
// Refactor this, Camera should be its own plugin and listen for events on the event system
use arara_camera::CameraController;

use crate::{EventLoop, Window};

pub fn run(mut app: App) {
    let mut mouse_pressed = false;

    let mut ev = app.world.get_non_send_resource_mut::<EventLoop>().unwrap();
    let event_loop = ev.take().unwrap();

    trace!("Entering winit event loop");
    event_loop.run(move |ev, _, control_flow| {
        match ev {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    if mouse_pressed {
                        let mut camera_controller =
                            app.world.get_resource_mut::<CameraController>().unwrap();
                        camera_controller.process_mouse(delta.0, delta.1);
                    }
                }
                _ => (),
            },
            Event::WindowEvent { event, window_id } => {
                if window_id != get_primary_window_id(&app.world) {
                    trace!("recieved event for unknown window_id");
                    return;
                }

                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
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
                        let mut camera_controller =
                            app.world.get_resource_mut::<CameraController>().unwrap();
                        camera_controller.process_keyboard(key, state);
                    }
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state,
                        ..
                    } => {
                        mouse_pressed = state == ElementState::Pressed;
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let mut camera_controller =
                            app.world.get_resource_mut::<CameraController>().unwrap();
                        camera_controller.process_scroll(&delta);
                    }
                    WindowEvent::Resized(physical_size) => {
                        let mut camera_controller =
                            app.world.get_resource_mut::<CameraController>().unwrap();
                        camera_controller.resize_from_size(physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let mut camera_controller =
                            app.world.get_resource_mut::<CameraController>().unwrap();
                        camera_controller.resize_from_size(*new_inner_size);
                    }
                    _ => return,
                }
            }
            Event::MainEventsCleared => {
                app.update();
            }
            _ => (),
        }

        update_camera(&mut app.world);
        *control_flow = ControlFlow::Poll;
    });
}

fn get_primary_window_id(world: &World) -> WindowId {
    let window = world.get_non_send_resource::<Window>().unwrap();
    window.display().gl_window().window().id()
}

fn update_camera(world: &mut World) {
    let world = world.cell();
    let mut camera_controller = world.get_non_send_mut::<CameraController>().unwrap();
    let time = world.get_resource::<Time>().unwrap();
    camera_controller.update_camera(time.delta());
}