use arara_ecs::system::{NonSend, Res};
use arara_window::Window;
use glium::implement_uniform_block;

use crate::{BPLight, ExtractedView, TextureBuffer};



#[derive(Debug, Default, Clone, Copy)]
struct CameraUniformBuffer {
    u_pv_matrix: [[f32; 4]; 4],
}

impl CameraUniformBuffer {
    fn new(u_pv_matrix: [[f32; 4]; 4]) -> Self {
        Self { u_pv_matrix }
    }
}

implement_uniform_block!(CameraUniformBuffer, u_pv_matrix);

#[derive(Copy, Clone)]
struct TextureUniformBuffer<'a> {
    tex: [glium::texture::TextureHandle<'a>; 5],
}

implement_uniform_block!(TextureUniformBuffer<'a>, tex);

#[derive(Copy, Clone)]
struct BPLightUniformBuffer {
    pub u_camera_pos: [f32; 3],
    pub u_light_pos: [f32; 3],
}

implement_uniform_block!(BPLightUniformBuffer, u_camera_pos, u_light_pos);

pub(crate) fn queue_uniforms(
    window: NonSend<Window>,
    light: Res<BPLight>,
    view: Res<ExtractedView>,
    texture_buffer: NonSend<TextureBuffer>,
) {
    let display = window.display();
    let pv_matrix: [[f32; 4]; 4] = view.pv_matrix.to_cols_array_2d();
    let camera_uniform_buffer =
        glium::uniforms::UniformBuffer::new(display, CameraUniformBuffer::new(pv_matrix)).unwrap();

    let texture_uniform_buffer =
        glium::uniforms::UniformBuffer::new(display, texture_buffer.texture_uniform_buffer())
            .unwrap();

    let bplight_uniform_buffer = glium::uniforms::UniformBuffer::new(
        display,
        BPLightUniformBuffer {
            u_camera_pos: view.position.into(),
            u_light_pos: light.position.into(),
        },
    )
    .unwrap();

    glium::uniform! {
        camera: &camera_uniform_buffer,
        bplight: &bplight_uniform_buffer,
        samplers: &texture_uniform_buffer,
    };
}