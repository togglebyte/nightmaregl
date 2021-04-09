#![deny(missing_docs)]
use gl33::global_loader::*;
use gl33::*;
use num_traits::cast::NumCast;

use crate::Texture;

/// Frame buffer
///
/// ```
/// use nightmaregl::Framebuffer;
/// # use nightmaregl::Texture;
/// # fn run(texture: Texture<f32>) {
/// let fb = Framebuffer::new(texture);
/// fb.bind();
///
/// // do some rendering to the frame buffer
/// # }
/// ```
pub struct Framebuffer<T: NumCast + Copy> {
    id: u32,
    pub(crate) texture: Texture<T>
}

impl<T: NumCast + Copy> Framebuffer<T> {
    /// Create a new framebuffer
    pub fn new(texture: Texture<T>) -> Self {
        let mut id = 0;
        unsafe { glGenFramebuffers(1, &mut id) };
        Self { 
            id,
            texture,
        }
    }

    /// Bind this framebuffer, making all subsequent draw calls act
    /// on this buffer and it's texture.
    pub fn bind(&self) {
        unsafe { glBindFramebuffer(GL_FRAMEBUFFER, self.id) };
    }

    /// Unbind this buffer and texture.
    pub fn unbind(&self) {
        unsafe { glBindFramebuffer(GL_FRAMEBUFFER, 0) };
    }

    /// Attach a texture to this frame buffer to render to.
    pub fn attach_texture(&self, texture: &Texture<T>) {
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

    /// Get a reference to the texture owned
    /// by the framebuffer.
    pub fn texture(&self) -> &Texture<T> {
        &self.texture
    }
}

impl<T: NumCast + Copy> Drop for Framebuffer<T> {
    // If the framebuffer is currently bound,
    // framebuffer zero will be bound instead when
    // this buffer is deleted.
    fn drop(&mut self) {
        unsafe { glDeleteFramebuffers(1, &mut self.id) }
    }
}
