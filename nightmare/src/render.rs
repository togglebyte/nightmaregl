use gl33::global_loader::*;
use gl33::*;

use crate::Color;

#[derive(Debug, Copy, Clone)]
pub struct UniformLocation(pub(crate) i32);

/// Clear the frame buffer.
/// ```
/// use nightmaregl::Color;
/// # use nightmaregl::Context;
/// # fn run(context: &mut Context) {
/// loop {
///     context.clear(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
///     context.swap_buffers();
/// }
/// # }
/// ```
pub fn clear(color: Color) {
    unsafe {
        glClearColor(color.r, color.g, color.b, color.a);
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    }
}

pub fn instanced_draw(vertex_count: usize, instance_count: usize) {
    unsafe {
        glDrawArraysInstanced(
            GL_TRIANGLE_STRIP,
            0,
            vertex_count as i32,
            instance_count as i32,
        )
    };
}
