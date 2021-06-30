//! Window and associated to window rendering context related functions.

use crate::get_context;

use crate::color::Color;

// miniquad is re-exported for the use in combination with `get_internal_gl`
pub use miniquad;

pub use miniquad::conf::Conf;

/// Block execution until the next frame.
pub fn next_frame() -> crate::exec::FrameFuture {
    crate::exec::FrameFuture
}

/// Fill window background with solid color.
/// Note: even when "clear_background" was not called explicitly
/// screen will be cleared at the beginning of the frame.
///
/// Also sets the depth to `f32::MAX`, so everything draws on top.
/// (I don't know why it has to be MAX; it seems like it should be MIN. Oh well.)
pub fn clear_background(color: Color) {
    let context = get_context();

    context.gl.clear(&mut context.quad_context, color);
}

#[doc(hidden)]
pub fn gl_set_drawcall_buffer_capacity(max_vertices: usize, max_indices: usize) {
    let context = get_context();
    context
        .gl
        .update_drawcall_capacity(&mut context.quad_context, max_vertices, max_indices);
}

pub struct InternalGlContext<'a> {
    pub quad_context: &'a mut miniquad::Context,
    pub quad_gl: &'a mut crate::quad_gl::QuadGl,
}

impl<'a> InternalGlContext<'a> {
    /// Draw all the batched stuff and reset the internal state cache
    /// May be helpful for combining macroquad's drawing with raw miniquad/opengl calls
    pub fn flush(&mut self) {
        get_context().perform_render_passes();
    }
}

pub unsafe fn get_internal_gl<'a>() -> InternalGlContext<'a> {
    let context = get_context();

    InternalGlContext {
        quad_context: &mut context.quad_context,
        quad_gl: &mut context.gl,
    }
}

pub fn screen_width() -> f32 {
    let context = get_context();

    context.screen_width
}

pub fn screen_height() -> f32 {
    let context = get_context();

    context.screen_height
}
