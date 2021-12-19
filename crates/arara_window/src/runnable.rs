use arara_utils::tracing::trace;
use glium::{
    self,
    glutin::{event::*, event_loop::ControlFlow},
};

use arara_app::{App, AppExit, Events, ManualEventReader};
use arara_input::{keyboard::KeyboardInput, mouse::*};

use crate::{converters, event::*, EventLoop, Window};

pub fn run(mut app: App) {
    let mut app_exit_event_reader = ManualEventReader::<AppExit>::default();
    // let mut mouse_pressed = false;

    let mut ev = app.world.get_non_send_resource_mut::<EventLoop>().unwrap();
    let event_loop = ev.take().unwrap();

    trace!("Entering winit event loop");
    event_loop.run(move |ev, _, control_flow| {
        // update_camera(&mut app.world);
        *control_flow = ControlFlow::Poll;

        // Exit on [App::AppExit] event
        if let Some(app_exit_events) = app.world.get_resource_mut::<Events<AppExit>>() {
            if app_exit_event_reader
                .iter(&app_exit_events)
                .next_back()
                .is_some()
            {
                *control_flow = ControlFlow::Exit;
            }
        }

        match ev {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    let mut mouse_motion_events =
                        app.world.get_resource_mut::<Events<MouseMotion>>().unwrap();
                    mouse_motion_events.send(MouseMotion {
                        delta: (delta.0 as f32, delta.1 as f32),
                    });
                }
                _ => (),
            },
            Event::WindowEvent { event, window_id } => {
                let world = app.world.cell();
                let mut window = world.get_non_send_mut::<Window>().unwrap();

                if window_id != window.display().gl_window().window().id() {
                    trace!("recieved event for unknown window_id");
                    return;
                }

                match event {
                    WindowEvent::Resized(size) => {
                        let mut resize_events =
                            world.get_resource_mut::<Events<WindowResized>>().unwrap();
                        window.update_actual_size_from_backend(size.width, size.height);
                        resize_events.send(WindowResized {
                            width: size.width,
                            height: size.height,
                        });
                    }
                    WindowEvent::CloseRequested => {
                        let mut window_close_requested_events = world
                            .get_resource_mut::<Events<WindowCloseRequested>>()
                            .unwrap();
                        window_close_requested_events.send(WindowCloseRequested);
                    }
                    WindowEvent::KeyboardInput { ref input, .. } => {
                        let mut keyboard_input_events =
                            world.get_resource_mut::<Events<KeyboardInput>>().unwrap();
                        keyboard_input_events.send(converters::convert_keyboard_input(input));
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let mut cursor_moved_events =
                            world.get_resource_mut::<Events<CursorMoved>>().unwrap();
                        // let position = position.to_logical(winit_window.scale_factor());
                        let position = (position.x as f32, position.y as f32);
                        window.update_cursor_position_from_backend(Some(position));
                        cursor_moved_events.send(CursorMoved { position });
                    }
                    WindowEvent::CursorEntered { .. } => {
                        let mut cursor_entered_events =
                            world.get_resource_mut::<Events<CursorEntered>>().unwrap();
                        cursor_entered_events.send(CursorEntered);
                    }
                    WindowEvent::CursorLeft { .. } => {
                        let mut cursor_left_events =
                            world.get_resource_mut::<Events<CursorLeft>>().unwrap();
                        window.update_cursor_position_from_backend(None);
                        cursor_left_events.send(CursorLeft);
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        let mut mouse_button_input_events = world
                            .get_resource_mut::<Events<MouseButtonInput>>()
                            .unwrap();
                        mouse_button_input_events.send(MouseButtonInput {
                            button: converters::convert_mouse_button(button),
                            state: converters::convert_element_state(state),
                        });
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            let mut mouse_wheel_input_events =
                                world.get_resource_mut::<Events<MouseWheel>>().unwrap();
                            mouse_wheel_input_events.send(MouseWheel {
                                unit: MouseScrollUnit::Line,
                                x,
                                y,
                            });
                        }
                        MouseScrollDelta::PixelDelta(p) => {
                            let mut mouse_wheel_input_events =
                                world.get_resource_mut::<Events<MouseWheel>>().unwrap();
                            mouse_wheel_input_events.send(MouseWheel {
                                unit: MouseScrollUnit::Pixel,
                                x: p.x as f32,
                                y: p.y as f32,
                            });
                        }
                    },
                    WindowEvent::ReceivedCharacter(c) => {
                        let mut char_input_events = world
                            .get_resource_mut::<Events<ReceivedCharacter>>()
                            .unwrap();
                        char_input_events.send(ReceivedCharacter { char: c })
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        {
                            let mut scale_factor_change_events = world
                                .get_resource_mut::<Events<WindowScaleFactorChanged>>()
                                .unwrap();
                            scale_factor_change_events
                                .send(WindowScaleFactorChanged { scale_factor });
                        }
                        {
                            let mut resize_events =
                                world.get_resource_mut::<Events<WindowResized>>().unwrap();
                            resize_events.send(WindowResized {
                                width: new_inner_size.width,
                                height: new_inner_size.height,
                            });
                        }
                        window.update_actual_size_from_backend(
                            new_inner_size.width,
                            new_inner_size.height,
                        );
                    }
                    WindowEvent::Focused(focused) => {
                        window.update_focused_status_from_backend(focused);
                        let mut focused_events =
                            world.get_resource_mut::<Events<WindowFocused>>().unwrap();
                        focused_events.send(WindowFocused { focused });
                    }
                    WindowEvent::DroppedFile(path_buf) => {
                        let mut events =
                            world.get_resource_mut::<Events<FileDragAndDrop>>().unwrap();
                        events.send(FileDragAndDrop::DroppedFile { path_buf });
                    }
                    WindowEvent::HoveredFile(path_buf) => {
                        let mut events =
                            world.get_resource_mut::<Events<FileDragAndDrop>>().unwrap();
                        events.send(FileDragAndDrop::HoveredFile { path_buf });
                    }
                    WindowEvent::HoveredFileCancelled => {
                        let mut events =
                            world.get_resource_mut::<Events<FileDragAndDrop>>().unwrap();
                        events.send(FileDragAndDrop::HoveredFileCancelled);
                    }
                    WindowEvent::Moved(position) => {
                        let position = (position.x, position.y);
                        let mut events = world.get_resource_mut::<Events<WindowMoved>>().unwrap();
                        events.send(WindowMoved { position });
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                app.update();
            }
            _ => (),
        }
    });
}
