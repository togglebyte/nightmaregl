#![deny(missing_docs)]
//! OpenGL framebuffer.
//!
//! For more information:
//!
//! [https://www.khronos.org/opengl/wiki/Framebuffer_Object](https://www.khronos.org/opengl/wiki/Framebuffer_Object)
//!
//! ```
//! use nightmaregl::framebuffer::{Framebuffer, FramebufferTarget};
//!
//! // Create a framebuffer that can be both read from, and written to.
//! let fb = Framebuffer::new(FramebufferTarget::Both);
//! ```
use gl33::global_loader::*;
use gl33::*;
use num_traits::cast::NumCast;

use crate::Texture;

/// Framebuffer target.
/// For more information see:
///
/// [https://www.khronos.org/opengl/wiki/Framebuffer_Object#Framebuffer_Object_Structure](https://www.khronos.org/opengl/wiki/Framebuffer_Object#Framebuffer_Object_Structure)
#[derive(Debug, Copy, Clone)]
pub enum FramebufferTarget {
    /// GL_READ_FRAMEBUFFER
    Read,

    /// GL_WRITE_FRAMEBUFFER
    Draw,

    /// GL_FRAMEBUFFER
    Both,
}

impl FramebufferTarget {
    fn to_gl(self) -> GLenum {
        match self {
            FramebufferTarget::Read => GL_READ_FRAMEBUFFER,
            FramebufferTarget::Draw => GL_DRAW_FRAMEBUFFER,
            FramebufferTarget::Both => GL_FRAMEBUFFER,
        }
    }
}

impl Default for FramebufferTarget {
    fn default() -> Self {
        FramebufferTarget::Both
    }
}

/// Frame buffer
///
/// When rendering to a framebuffer the Y axis will be inverted.
/// To fix this it is possible to call `viewport.flip_y` before rendering.
///
/// ```
/// use nightmaregl::Framebuffer;
/// # use nightmaregl::Texture;
/// # fn run(texture: Texture<f32>) {
/// let fb = Framebuffer::new();
/// fb.attach_texture(&texture);
/// fb.bind();
///
/// // do some rendering to the frame buffer
/// # }
/// ```
pub struct Framebuffer {
    id: u32,
    target: FramebufferTarget,
}

impl Framebuffer {
    /// Create a new framebuffer
    pub fn new(target: FramebufferTarget) -> Self {
        let mut id = 0;
        unsafe { glGenFramebuffers(1, &mut id) };
        Self { id, target }
    }

    /// Bind this framebuffer, making all subsequent draw calls act
    /// on this buffer.
    pub fn bind(&mut self) {
        unsafe { glBindFramebuffer(self.target.to_gl(), self.id) };
    }

    /// Bind this framebuffer to a specific target.
    pub fn bind_target(&mut self, target: FramebufferTarget) {
        self.target = target;
        self.bind();
    }

    /// Unbind this buffer.
    /// This will bind the default framebuffer.
    pub fn unbind(&self) {
        unsafe { glBindFramebuffer(GL_FRAMEBUFFER, 0) };
    }

    /// Attach a texture to this frame buffer to render to.
    pub fn attach_texture<T: Copy + NumCast>(&mut self, texture: &Texture<T>) {
        self.bind();
        texture.bind();

        unsafe {
            glFramebufferTexture2D(
                GL_FRAMEBUFFER,
                GL_COLOR_ATTACHMENT0,
                GL_TEXTURE_2D,
                texture.id(),
                0,
            )
        };

        self.unbind();
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Drop for Framebuffer {
    // If the framebuffer is currently bound,
    // framebuffer zero will be bound instead when
    // this buffer is deleted.
    fn drop(&mut self) {
        unsafe { glDeleteFramebuffers(1, &mut self.id) }
    }
}
