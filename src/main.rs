mod camera;
mod shaders;
mod geometry;
mod transform;

#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;

use glium::Surface;
use glium::glutin;

use transform::TransformBuilder;

fn main() {
    let size = glium::glutin::dpi::LogicalSize::new(1024, 768);
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("CGR");
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = image::load(
        Cursor::new(&include_bytes!("./joaozinho.png")),
        image::ImageFormat::Png,
    ).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut perspective = camera::Perspective::new(size.width, size.height, cgmath::Deg(60.0), 0.1, 1024.0);
    let mut camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
    let mut camera_controller = camera::CameraController::new(1.0, 0.4);
    let mut mouse_pressed = false;

    let sphere = geometry::sphere::Sphere::new(32, 16);

    let transforms = [
        TransformBuilder::new().build(),
        TransformBuilder::new()
            .translate(0.0, 1.5, 0.0)
            .scale(0.8)
            .build(),
        TransformBuilder::new()
            .scale(0.4)
            .translate(0.0, 2.3, 0.0)
            .build(),
        TransformBuilder::new()
            .scale(0.1)
            .translate(0.0, 2.3, 0.4)
            .build(),
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &sphere.vertices).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &sphere.indices).unwrap();
    
    let per_instance = glium::VertexBuffer::new(&display, &transforms).unwrap();

    let shaders = shaders::Shaders::default();
    let program = glium::Program::from_source(&display, shaders.vertex_shader, shaders.fragment_shader, None).unwrap();

    event_loop.run(move |ev, _, control_flow| {

        // let (width, height) = frame.get_dimensions();
        let perspective_matrix: [[f32; 4]; 4] = perspective.calc_matrix().into();
        let view: [[f32; 4]; 4] = camera.calc_matrix().into();
        let light = [0.0, 0.0, 0.0f32];

        let uniforms = uniform! {
            view: view,
            perspective: perspective_matrix,
            u_light: light,
            tex: &texture,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let mut frame = display.draw();
        frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        frame.draw((&vertex_buffer, per_instance.per_instance().unwrap()), &indices, &program, &uniforms, &params).unwrap();
        // frame.draw((&vertex_buffer, per_instance.per_instance().unwrap()), &indices, &program, &uniforms_2, &params).unwrap();
        frame.finish().unwrap();

        use glutin::event::*;
        match ev {
            Event::DeviceEvent { ref event, .. } => match event {
                DeviceEvent::Key(
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    }
                ) => {
                    camera_controller.process_keyboard(*key, *state);
                }
                DeviceEvent::MouseWheel { delta, .. } => {
                    camera_controller.process_scroll(delta);
                }
                DeviceEvent::Button {
                    button: 1, // Left Mouse Button
                    state,
                } => {
                    mouse_pressed = *state == ElementState::Pressed;
                }
                DeviceEvent::MouseMotion { delta } => {
                    if mouse_pressed {
                        camera_controller.process_mouse(delta.0, delta.1);
                    }
                }
                _ => (),
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                WindowEvent::Resized(physical_size) => {
                    perspective.resize_from_size(physical_size);
                },
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    perspective.resize_from_size(*new_inner_size);
                }
                _ => return,
            },
            _ => (),
        }

        let dt = std::time::Duration::from_nanos(16_666_667);
        let next_frame_time = std::time::Instant::now() + dt;
        camera_controller.update_camera(&mut camera, dt);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}
