#![deny(missing_docs)]
//! A texture can either be an image uploaded to the gpu, or it can something a frame buffer
//! renders to.
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use num_traits::cast::NumCast;
use gl33::global_loader::*;
use gl33::*;
use png::{ColorType, Decoder, OutputInfo};

use crate::{Position, Size};
use crate::errors::{Result, NightmareError};

// -----------------------------------------------------------------------------
//     - Output info extension -
// -----------------------------------------------------------------------------
trait OutputInfoExt {
    fn pixel_count(&self, pixel_size: usize) -> usize;
}

impl OutputInfoExt for OutputInfo {
    fn pixel_count(&self, pixel_size: usize) -> usize {
        self.width as usize * self.height as usize * pixel_size
    }
}

#[derive(Debug, Copy, Clone)]
/// Texture format.
/// Currently only supports RGBA and Red
pub enum Format {
    /// RGBA values. This is most likely the format to use,
    /// unless dealing with fonts.
    Rgba,
    /// This is most likely used with text
    Red,
}

impl Format {
    fn to_format(&self) -> PixelFormat {
        match self {
            Format::Rgba => GL_RGBA,
            Format::Red => GL_RED,
        }
    }

    fn to_internal_format(&self) -> i32 {
        match self {
            Format::Rgba => GL_RGBA8.0 as i32,
            Format::Red => GL_RED.0 as i32,
        }
    }

    fn size(&self) -> usize {
        match self {
            Format::Rgba => 4,
            Format::Red => 1,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Wrap -
// -----------------------------------------------------------------------------
/// Texture wrapping
pub enum Wrap {
    /// Repeat
    Repeat,
    /// Don't wrap the texture
    NoWrap,
}

// -----------------------------------------------------------------------------
//     - Texture filter -
// -----------------------------------------------------------------------------
/// Texture filter
pub enum Filter {
    /// Useful for 2D pixel art
    Nearest,
    /// Useful for textures on 3D objects
    Linear,
}

// -----------------------------------------------------------------------------
//     - Texture builder -
// -----------------------------------------------------------------------------
/// Texture builder that is missing a format.
pub struct NoFormat;
/// A texture builder that has a format set.
pub struct WithFormat(Format);

/// A texture builder.
/// To create a texture builder use [`Texture::new`].
pub struct TextureBuilder<T>(u32, T);

impl TextureBuilder<NoFormat> {
    fn new() -> Self {
        let mut texture_id = 0;

        unsafe {
            glGenTextures(1, &mut texture_id);
            glBindTexture(GL_TEXTURE_2D, texture_id);
        }

        Self(texture_id, NoFormat)
    }

    /// One bit aligned, red channel only texture.
    pub(crate) fn empty_text<T: Copy + NumCast>(self, size: impl Into<Size<T>>) -> Texture<T> {
        let size = size.into();
        let format = Format::Red;

        unsafe {
            let size = size.to_i32();

            // Set alignment to one
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);

            // Fill the texture with black pixels
            let black = vec![0u8; size.width as usize * size.height as usize];

            glTexImage2D(
                GL_TEXTURE_2D,
                0, // Level,
                format.to_internal_format(),
                size.width,
                size.height,
                0, // Border
                format.to_format(),
                GL_UNSIGNED_BYTE,
                black.as_ptr().cast(),
            );

            // Restore alignment to four
            glPixelStorei(GL_UNPACK_ALIGNMENT, 4);
        };

        let texture = Texture { id: self.0, size, format };

        texture.min_filter(Filter::Nearest);
        texture.mag_filter(Filter::Nearest);
        texture.wrap_x(Wrap::NoWrap);
        texture.wrap_y(Wrap::NoWrap);

        texture
    }

    /// Set the texture format.
    pub fn with_format(self, format: Format) -> TextureBuilder<Format> {
        TextureBuilder(self.0, format)
    }
}


impl TextureBuilder<Format> {
    /// Create a texture with some data.
    ///
    /// ```
    /// use nightmaregl::texture::{Texture, Format};
    /// use nightmaregl::Size;
    /// # fn run(data: &[u8]) {
    /// let texture = Texture::<f32>::new()
    ///     .with_format(Format::Rgba)
    ///     .with_data(data, Size::new(32, 32));
    /// # }
    /// ```
    pub fn with_data<T: Copy + NumCast>(self, data: &[u8], size: impl Into<Size<T>>) -> Texture<T> {
        let size = size.into().to_i32();
        debug_assert_eq!(data.len(), size.width as usize * size.height as usize * self.1.size());

        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0, // Level,
                self.1.to_internal_format(),
                size.width,
                size.height,
                0, // Border
                self.1.to_format(),
                GL_UNSIGNED_BYTE,
                data.as_ptr().cast(),
            )
        };

        let texture = Texture { id: self.0, size: size.cast(), format: self.1 };

        texture.min_filter(Filter::Nearest);
        texture.mag_filter(Filter::Nearest);
        texture.wrap_x(Wrap::NoWrap);
        texture.wrap_y(Wrap::NoWrap);

        texture
    }

    /// This should probably only ever be used as a framebuffer texture.
    /// Because this could contain rubbish data since it's not using initialized values.
    pub fn with_no_data<T: Copy + NumCast>(self, size: impl Into<Size<T>>) -> Texture<T> {
        let size = size.into().to_i32();

        unsafe {
            glTexImage2D(
                GL_TEXTURE_2D,
                0, // Level,
                self.1.to_internal_format(),
                size.width,
                size.height,
                0, // Border
                self.1.to_format(),
                GL_UNSIGNED_BYTE,
                std::ptr::null(),
            )
        };

        let texture = Texture { id: self.0, size: size.cast(), format: self.1 };

        texture.min_filter(Filter::Nearest);
        texture.mag_filter(Filter::Nearest);
        texture.wrap_x(Wrap::NoWrap);
        texture.wrap_y(Wrap::NoWrap);

        texture
    }
}

// -----------------------------------------------------------------------------
//     - Texture -
// -----------------------------------------------------------------------------
/// Textures are created using the [`Self::new`] function.
/// When a new texture is created it is automatically bound.
///
/// ```
/// # use nightmaregl::texture::{Filter, Texture};
/// # use nightmaregl::Size;
/// # fn run(data: Vec<u8>) {
/// let texture = Texture::default_with_data(Size::new(800.0, 600.0), &data);
/// # }
/// ```
#[derive(Debug)]
pub struct Texture<T: Copy + NumCast> {
    id: u32,
    pub(crate) size: Size<T>,
    format: Format,
}

impl<T: Copy + NumCast> Texture<T> {
    /// Create a [TextureBuilder](crate::texture::TextureBuilder).
    pub fn new() -> TextureBuilder<NoFormat> {
        TextureBuilder::new()
    }

    unsafe fn align_for_read(&self) {
        match self.format {
            Format::Rgba => { glPixelStorei(GL_PACK_ALIGNMENT, 4) }
            Format::Red => { glPixelStorei(GL_PACK_ALIGNMENT, 1) }
        }
    }

    unsafe fn align_for_write(&self) {
        match self.format {
            Format::Rgba => { glPixelStorei(GL_UNPACK_ALIGNMENT, 4) }
            Format::Red => { glPixelStorei(GL_UNPACK_ALIGNMENT, 1) }
        }
    }

    unsafe fn align_default(&self) {
        glPixelStorei(GL_UNPACK_ALIGNMENT, 4);
        glPixelStorei(GL_PACK_ALIGNMENT, 4);
    }

    /// Create a texure with some default data.
    /// Assumes RGBA.
    pub fn default_with_data(size: impl Into<Size<T>>, data: &[u8]) -> Texture<T> {
        let builder = TextureBuilder::new().with_format(Format::Rgba);
        builder.with_data(data, size)
    }

    fn wrap(&self, wrap: Wrap, target: GLenum) -> &Self {
        let wrap = match wrap {
            Wrap::Repeat => GL_REPEAT.0,
            Wrap::NoWrap => GL_CLAMP_TO_EDGE.0,
        } as i32;

        unsafe { glTexParameteri(GL_TEXTURE_2D, target, wrap) };

        self
    }

    fn filter(&self, filter: Filter, target: GLenum) -> &Self {
        let filter = match filter {
            Filter::Nearest => GL_NEAREST.0,
            Filter::Linear => GL_LINEAR.0,
        } as i32;

        unsafe { glTexParameteri(GL_TEXTURE_2D, target, filter) };

        self
    }

    pub(crate) fn id(&self) -> u32 {
        self.id
    }

    /// Set the texture wrapping on the x axis.
    pub fn wrap_x(&self, wrap: Wrap) -> &Self {
        self.wrap(wrap, GL_TEXTURE_WRAP_S)
    }

    /// Set the texture wrapping on the y axis.
    pub fn wrap_y(&self, wrap: Wrap) -> &Self {
        self.wrap(wrap, GL_TEXTURE_WRAP_T)
    }

    /// Set the min filter.
    /// This is set to `Nearest` by default.
    pub fn min_filter(&self, filter: Filter) -> &Self {
        self.filter(filter, GL_TEXTURE_MIN_FILTER)
    }

    /// Set the mag filter.
    /// This is set to `Nearest` by default.
    pub fn mag_filter(&self, filter: Filter) -> &Self {
        self.filter(filter, GL_TEXTURE_MAG_FILTER)
    }

    /// Bind the texture
    pub fn bind(&self) {
        unsafe { glBindTexture(GL_TEXTURE_2D, self.id) };
    }

    /// Get the size of the texture.
    pub fn size(&self) -> Size<T> {
        self.size
    }

    /// Write data to a region of a texture
    pub fn write_region(&self, position: Position<T>, size: Size<T>, data: &[u8]) {
        self.bind();

        debug_assert!(data.len() <= size.cast::<usize>().width * size.cast::<usize>().height * self.format.size());

        if let Format::Red = self.format {
            unsafe { self.align_for_write() };
        }

        let position = position.to_i32();
        let size = size.to_i32();

        unsafe {
            glTexSubImage2D(
                GL_TEXTURE_2D,
                0, // Level,
                position.x,
                position.y,
                size.width,
                size.height,
                self.format.to_format(),
                GL_UNSIGNED_BYTE,
                data.as_ptr().cast(),
            );
        }

        if let Format::Red = self.format {
            unsafe { self.align_default() };
        }
    }

    /// Read pixels out of a texture.
    pub fn read_pixels(
        &self,
        position: Position<T>,
        size: Size<T>,
        output_buf: &mut [u8],
    ) {
        self.bind();

        if let Format::Red = self.format {
            unsafe { self.align_for_read() };
        }

        unsafe {
            let position = position.to_i32();
            let size = size.to_i32();
            glReadPixels(
                position.x,
                position.y,
                size.width,
                size.height,
                self.format.to_format(),
                GL_UNSIGNED_BYTE,
                output_buf.as_mut_ptr().cast(),
            )
        }

        if let Format::Red = self.format {
            unsafe { self.align_default() };
        }
    }

    /// Load a texture from disk.
    /// ```
    /// use nightmaregl::{Result, Texture};
    /// # fn run() -> Result<Texture<f32>> {
    /// let texture = Texture::from_disk("my_texture.png")?;
    /// # Ok(texture)
    /// # }
    /// ```
    pub fn from_disk(path: impl AsRef<Path>) -> Result<Self> {
        let path: &Path = path.as_ref();
        let file = File::open(path)?;

        let decoder = Decoder::new(file);
        let (info, mut reader) = decoder.read_info()?;

        // The size of a pixel in bytes.
        // RGB  = u8 u8 u8
        // RGBA = u8 u8 u8 u8
        let (pixel_size, format) = match info.color_type {
            ColorType::Grayscale => (1, Format::Red),
            // ColorType::RGB => (3, Format::Rgb),
            ColorType::RGBA => (4, Format::Rgba),
            _ => return Err(NightmareError::InvalidColorType),
        };

        let capacity = info.pixel_count(pixel_size);
        let mut bytes = Vec::with_capacity(capacity);
        unsafe { bytes.set_len(capacity) };

        reader.next_frame(&mut bytes)?;

        // Create an OpenGL texture associated
        // with the sprite.
        let size = Size::new(info.width, info.height).cast::<T>();
        let texture = Texture::<T>::new().with_format(format).with_data(&bytes, size);

        Ok(texture)
    }

    /// Write a texture to disk.
    pub fn write_to_disk(&self, dst: impl AsRef<Path>) -> Result<()> {
        let size = self.size.to_i32();
        let mut output_buf = vec![0u8; (size.width * size.height) as usize * 4];

        unsafe {
            glReadPixels(
                0,
                0,
                size.width,
                size.height,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                output_buf.as_mut_ptr().cast(),
            );
        }

        let file = File::create(dst.as_ref())?;
        let mut writer = BufWriter::new(file);
        let size = size.to_u32();
        let mut encoder = png::Encoder::new(&mut writer, size.width, size.height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&output_buf)?;

        Ok(())
    }

}

// -----------------------------------------------------------------------------
//     - Drop texture -
// -----------------------------------------------------------------------------
impl<T: Copy + NumCast> Drop for Texture<T> {
    fn drop(&mut self) {
        unsafe { glDeleteTextures(1, &self.id) };
    }
}
