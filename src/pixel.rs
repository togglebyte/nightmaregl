use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::{Color, Size};

// -----------------------------------------------------------------------------
//     - Pixel -
// -----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
/// A pixel, as a set of rgba colours.
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    /// An all white pixel
    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    /// An all black pixel
    pub fn black() -> Self {
        Self::default()
    }

    /// A transparent pixel (all values set to zero)
    pub fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl From<[u8; 4]> for Pixel {
    fn from(color: [u8; 4]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }
}

impl From<Color> for Pixel {
    fn from(color: Color) -> Self {
        Self {
            r: (255.0 * color.r) as u8,
            g: (255.0 * color.g) as u8,
            b: (255.0 * color.b) as u8,
            a: (255.0 * color.a) as u8,
        }
    }
}

// -----------------------------------------------------------------------------
//     - Pixel container -
// -----------------------------------------------------------------------------
/// Holds a bunch of pixels.
/// This is useful because of the `as_bytes` method which conveniently 
/// represents all pixels as a byte slice.
///
/// ```
/// use nightmaregl::Pixels;
/// # use nightmaregl::Size;
/// # fn run() {
/// let mut pixels = Pixels::from_size(Size::new(20, 20));
/// # }
/// ```
#[repr(transparent)]
pub struct Pixels(Vec<Pixel>);

impl Pixels {
    /// Allocate a collection of pixels.
    /// Note that this will not fill the buffer with values,
    /// so the length of this buffer is really 0.
    /// This is bad if this is passed to `write_region` of a `Texture` as 
    /// `glTexSubImage2D` is expecting to get width * height number of pixels,
    /// and if this isn't send, then the gpu will have to work with rubbish data.
    pub fn from_size(size: Size<usize>) -> Self {
        let cap = size.width * size.height;
        Self(Vec::with_capacity(cap))
    }

    /// Repeat the pixel width * height times.
    pub fn from_pixel(pixel: Pixel, size: Size<usize>) -> Self {
        let cap = size.width * size.height;
        Self(vec![pixel; cap])
    }

    /// Interpret the pixels as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.0)
    }

    /// Add a pixel
    pub fn push(&mut self, pixel: Pixel) {
        self.0.push(pixel);
    }
}

impl Index<usize> for Pixels {
    type Output = Pixel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Pixels {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Deref for Pixels {
    type Target = Vec<Pixel>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pixels {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
