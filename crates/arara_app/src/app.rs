use glium::glutin;
use glium::Surface;

use bevy_ecs;

pub struct App {
    world: bevy_ecs::world::World,
    event_loop: glutin::event_loop::EventLoop<()>,
    camera: arara_camera::Camera,
    camera_controller: arara_camera::CameraController,
    perspective: arara_camera::Perspective,
    display: glium::Display,
}

impl App {
    pub fn build() -> Self {
        let size = glium::glutin::dpi::LogicalSize::new(1024, 768);
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title("CGR");
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let mut perspective =
            Perspective::new(size.width, size.height, cgmath::Deg(60.0), 0.1, 1024.0);
        let mut camera = Camera::new((0.0, 5.0, 10.0), cgmath::Deg(-90.0), cgmath::Deg(-20.0));
        let mut camera_controller = CameraController::new(1.0, 0.4);

        Self {
            world: World::new(),
            event_loop,
            camera,
            camera_controller,
            perspective,
            display,
        }
    }

    pub fn new_entity(&self, shape: Shape) {
        self.world.spawn(shape);
    }

    pub fn run(&self) {
        let mut mouse_pressed = false;

        self.event_loop.run(move |ev, _, control_flow| {
            // let (width, height) = frame.get_dimensions();
            let perspective_matrix: [[f32; 4]; 4] = self.perspective.calc_matrix().into();
            let view: [[f32; 4]; 4] = self.camera.calc_matrix().into();
            let light = [0.0, 0.0, 0.0f32];

            let mut frame = self.display.draw();
            
            frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);

            for entity in self.world.entities() {
                
                let image = image::load(
                    Cursor::new(&include_bytes!("../../assets/textures/joaozinho.png")),
                    image::ImageFormat::Png,
                ).unwrap().to_rgba8();
                let image_dimensions = image.dimensions();
                let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
                let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
            
                let shape = Sphere::new(32, 16);
                let vertex_buffer = glium::VertexBuffer::new(&display, &shape.vertices).unwrap();
                let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &shape.indices).unwrap();
                
                let transforms = [
                    TransformBuilder::new()
                        .build(),
                    TransformBuilder::new()
                        .scale(0.8)
                        .translate(0.0, 1.5, 0.0)
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
                let per_instance = glium::VertexBuffer::new(&display, &transforms).unwrap();
            
                let shaders = Shaders::default();
                let program = glium::Program::from_source(&display, shaders.vertex_shader, shaders.fragment_shader, None).unwrap();

                frame
                    .draw(
                        (&vertex_buffer, per_instance.per_instance().unwrap()),
                        &indices,
                        &program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();
            }

            // frame.draw((&vertex_buffer, per_instance.per_instance().unwrap()), &indices, &program, &uniforms_2, &params).unwrap();
            frame.finish().unwrap();
            use glutin::event::*;
            match ev {
                Event::DeviceEvent { ref event, .. } => match event {
                    DeviceEvent::MouseMotion { delta } => {
                        if mouse_pressed {
                            self.camera_controller.process_mouse(delta.0, delta.1);
                        }
                    }
                    _ => (),
                },
                Event::WindowEvent { event, window_id }
                    if window_id == self.display.gl_window().window().id() =>
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
                            self.camera_controller.process_keyboard(key, state);
                        }
                        WindowEvent::MouseInput {
                            button: MouseButton::Left,
                            state,
                            ..
                        } => {
                            mouse_pressed = state == ElementState::Pressed;
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            self.camera_controller.process_scroll(&delta);
                        }
                        WindowEvent::Resized(physical_size) => {
                            self.perspective.resize_from_size(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.perspective.resize_from_size(*new_inner_size);
                        }
                        _ => return,
                    }
                }
                _ => (),
            }
            let dt = std::time::Duration::from_nanos(16_666_667);
            let next_frame_time = std::time::Instant::now() + dt;
            self.camera_controller.update_camera(&mut self.camera, dt);
            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        });
    }
}
