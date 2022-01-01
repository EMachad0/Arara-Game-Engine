use glium::{
    index, uniforms, vertex, Display, DrawError, DrawParameters, Frame, Program, Surface,
    SwapBuffersError,
};
use thiserror::Error;

use crate::Color;

pub struct TrackedFrame {
    frame: Frame,
}

impl TrackedFrame {
    /// Builds a new `Frame`.
    pub fn new(display: &Display) -> Self {
        Self {
            frame: display.draw(),
        }
    }

    /// Stop drawing, swap the buffers, and consume the Frame.
    ///
    /// See the documentation of `SwapBuffersError` about what is being returned.
    #[inline]
    pub fn finish(self) -> Result<(), SwapBuffersError> {
        self.frame.finish()
    }

    pub fn clear_color_and_depth(&mut self, color: Color) {
        let clear_color = (color.r(), color.g(), color.b(), color.a());
        self.frame.clear_color_and_depth(clear_color, 1.0);
    }

    pub fn draw<'a, 'b, V, I, U>(
        &mut self,
        vertex_buffer: V,
        index_buffer: I,
        program: &Program,
        uniforms: &U,
        draw_parameters: &DrawParameters<'_>,
    ) -> Result<(), DrawError>
    where
        I: Into<index::IndicesSource<'a>>,
        U: uniforms::Uniforms,
        V: vertex::MultiVerticesSource<'b>,
    {
        self.frame.draw(
            vertex_buffer,
            index_buffer,
            program,
            uniforms,
            draw_parameters,
        )
    }
}

#[derive(Error, Debug)]
pub enum FrameError {
    #[error("transparent")]
    SwapBuffersError(#[from] glium::SwapBuffersError),
    #[error(transparent)]
    DrawError(#[from] glium::DrawError),
}
