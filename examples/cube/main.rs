mod camera;
mod shaders;
mod geometry;

#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;

use glium::Surface;
use glium::glutin;
use geometry::vertex::Vertex;

fn main() {
    let size = glium::glutin::dpi::LogicalSize::new(1024, 768);
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("CGR");
    let cb = glutin::ContextBuilder::new()
        .with_depth_buffer(16);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = image::load(
        Cursor::new(&include_bytes!("../../joaozinho.png")),
        image::ImageFormat::Png,
    ).unwrap().to_rgba8();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let _shape = [
        Vertex { position: [-1.0,  1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
    ];

    let mut perspective = camera::Perspective::new(size.width, size.height, cgmath::Deg(60.0), 0.1, 1024.0);
    let mut camera = camera::Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
    let mut camera_controller = camera::CameraController::new(1.0, 0.4);
    let mut mouse_pressed = false;

    let cube = geometry::cube::Cuboid::default();

    let vertex_buffer = glium::VertexBuffer::new(&display, &cube.vertices).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &cube.indices).unwrap();
    let shaders = shaders::Shaders::default();
    let program = glium::Program::from_source(&display, shaders.vertex_shader, shaders.fragment_shader, None).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let mut frame = display.draw();
        
        // let (width, height) = frame.get_dimensions();
        let perspective_matrix: [[f32; 4]; 4] = perspective.calc_matrix().into();
        let view: [[f32; 4]; 4] = camera.calc_matrix().into();

        let model : [[f32; 4]; 4] = cube.model.into();

        let light = [0.0, 0.0, 0.0f32];

        let uniforms = uniform! {
            model: model,
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
            .. Default::default()
        };

        frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
        frame.finish().unwrap();

        let dt = std::time::Duration::from_nanos(16_666_667);
        let next_frame_time = std::time::Instant::now() + dt;

        camera_controller.update_camera(&mut camera, dt);
        
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

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
    });
}
