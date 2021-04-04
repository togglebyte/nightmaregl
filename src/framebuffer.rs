#![deny(missing_docs)]
use gl33::global_loader::*;
use gl33::*;
use num_traits::cast::NumCast;

use crate::Texture;

/// Frame buffer
///
/// ```
/// use nightmaregl::Framebuffer;
/// # fn run() {
/// let fb = Framebuffer::new();
/// fb.bind();
/// // do some rendering to the frame buffer
/// # }
/// ```
pub struct Framebuffer {
    id: u32,
}

impl Framebuffer {
    /// Create a new framebuffer
    pub fn new() -> Self {
        let mut id = 0;
        unsafe { glGenFramebuffers(1, &mut id) };
        Self { id }
    }

    /// Bind this framebuffer, making all subsequent draw calls act
    /// on this buffer.
    pub fn bind(&self) {
        unsafe { glBindFramebuffer(GL_FRAMEBUFFER, self.id) };
    }

    /// Unbind this buffer.
    pub fn unbind(&self) {
        unsafe { glBindFramebuffer(GL_FRAMEBUFFER, 0) };
    }

    /// Attach a texture to this frame buffer to render to.
    pub fn attach_texture<T: Copy + NumCast>(&self, texture: &Texture<T>) {
        self.bind();

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

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { glDeleteFramebuffers(1, &mut self.id) }
    }
}
