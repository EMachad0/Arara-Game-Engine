mod camera;

#[macro_use]
extern crate glium;
extern crate image;

use std::io::Cursor;

use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords, normal);

fn main() {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = image::load(
        Cursor::new(&include_bytes!("../joaozinho.png")),
        image::ImageFormat::Png,
    ).unwrap().to_rgba8();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let shape = [
        Vertex { position: [-1.0,  1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 1.0,  1.0, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 1.0, -1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
    ];

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;
        in vec2 tex_coords;
        out vec3 v_position;
        out vec3 v_normal;
        out vec2 v_tex_coords;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 perspective;
        
        uniform mat4 matrix;

        void main() {
            gl_Position = perspective * view * model * vec4(position, 1.0);
            v_normal = transpose(inverse(mat3(model))) * normal;
            v_position = gl_Position.xyz / gl_Position.w;
            v_tex_coords = tex_coords;
        }
    "#;

    let fragment_shader_src = r#"
        #version 150
        
        in vec3 v_position;
        in vec3 v_normal;
        in vec2 v_tex_coords;
        out vec4 color;

        uniform vec3 u_light;
        uniform sampler2D tex;
        
        const vec3 specular_color = vec3(1.0, 1.0, 1.0);

        void main() {
            vec3 diffuse_color = texture(tex, v_tex_coords).rgb;
            vec3 ambient_color = diffuse_color;
            
            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0); 
        }
    "#;

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let mut frame = display.draw();

        let perspective = {
            let (width, height) = frame.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f*aspect_ratio , 0.0,                            0.0, 0.0],
                [           0.0 ,   f,                            0.0, 0.0],
                [           0.0 , 0.0,      (zfar+znear)/(zfar-znear), 1.0],
                [           0.0 , 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
            ]
        };

        let view = camera::view_matrix(
            &[0.0, 0.0, -1.0],
            &[0.0, 0.0, 1.0],
            &[0.0, 1.0, 0.0],
        );

        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];

        let light = [0.0, 0.0, 0.0f32];

        let uniforms = uniform! {
            model: model,
            view: view,
            perspective: perspective,
            u_light: light,
            tex: &texture,
        };

        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
