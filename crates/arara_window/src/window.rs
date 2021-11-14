use glium::{
    self,
    glutin::{
        self, dpi,
        event::*,
        event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    },
};

use arara_camera::prelude::*;

use crate::window_props::WindowProps;

pub fn build_display(window_props: &WindowProps, event_loop: &EventLoop<()>) -> glium::Display {
    let size = dpi::LogicalSize::new(window_props.width, window_props.height);
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title(window_props.title.clone());
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(window_props.vsync);
    glium::Display::new(wb, cb, event_loop).unwrap()
}

pub fn run<F>(
    display: glium::Display,
    event_loop: EventLoop<()>,
    update_function: F,
    mut camera: Camera,
    mut camera_controller: CameraController,
    mut perspective: Perspective,
) where
    F: 'static + Fn(),
{
    let mut mouse_pressed = false;

    event_loop.run(move |ev, _, control_flow| {
        update_function();

        match ev {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    if mouse_pressed {
                        camera_controller.process_mouse(delta.0, delta.1);
                    }
                }
                _ => (),
            },
            Event::WindowEvent { event, window_id }
                if window_id == display.gl_window().window().id() =>
            {
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
                        camera_controller.process_scroll(&delta);
                    }
                    WindowEvent::Resized(physical_size) => {
                        perspective.resize_from_size(physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        perspective.resize_from_size(*new_inner_size);
                    }
                    _ => return,
                }
            }
            _ => (),
        }
        let dt = std::time::Duration::from_nanos(16_666_667);
        let next_frame_time = std::time::Instant::now() + dt;
        camera_controller.update_camera(&mut camera, dt);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
