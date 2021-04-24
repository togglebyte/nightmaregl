#![deny(missing_docs)]
//! Represent `Pixel`s as bytes.
//!
//! ```
//! use nightmaregl::pixels::{Pixel, Pixels};
//! let green = Pixel { g: 255, ..Default::default() };
//! let red = Pixel { r: 255, ..Default::default() };
//! let pixels = Pixels::new([green, red]);
//!
//! let bytes = pixels.as_bytes();
//! ```
use crate::{Color, Position, Size};
use std::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

mod region;

pub use region::{Region, RegionMut};

// -----------------------------------------------------------------------------
//     - Pixel -
// -----------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
/// A pixel, as a set of rgba colours.
pub struct Pixel {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
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

// -----------------------------------------------------------------------------
//     - Pixel trait impl -
// -----------------------------------------------------------------------------
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

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:03} {:03} {:03} {:03}", self.r, self.g, self.b, self.a)
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
/// use nightmaregl::pixels::Pixels;
/// # use nightmaregl::Size;
/// # fn run() {
/// let mut pixels = Pixels::from_size(Size::new(20, 20));
/// # }
/// ```
#[derive(Debug)]
pub struct Pixels(Vec<Pixel>);

impl Pixels {
    /// Create a new `Pixels` from a byte vec.
    pub fn new(inner: impl Into<Vec<Pixel>>) -> Self {
        Self(inner.into())
    }

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

    /// Get an iterator over a region
    pub fn region(
        &self,
        row_width: usize,
        position: Position<usize>,
        size: Size<usize>,
    ) -> Region<Pixel> {
        debug_assert!(row_width >= size.width + position.x);

        let region = self
            .chunks_exact(row_width)
            .skip(position.y)
            .take(size.height)
            .map(|c| &c[position.x..size.width + position.x])
            .collect::<Vec<_>>();

        Region { inner: region }
    }

    /// Get an iterator over a region
    pub fn region_mut(
        &mut self,
        row_width: usize,
        position: Position<usize>,
        size: Size<usize>,
    ) -> RegionMut<Pixel> {
        debug_assert!(row_width >= size.width + position.x);

        let region = self
            .chunks_exact_mut(row_width)
            .skip(position.y)
            .take(size.height)
            .map(|c| &mut c[position.x..size.width + position.x])
            .collect::<Vec<_>>();

        RegionMut { inner: region }
    }

    /// Write a region
    pub fn write_region(
        &mut self,
        row_width: usize,
        position: Position<usize>,
        region: Region<Pixel>,
    ) {

        for (i, row) in region.rows().enumerate() {
            let y = (position.y + i) * row_width;
            let index = y + position.x;
            let dest = &mut self.0[index..index + row.len()];
            dest.copy_from_slice(row);
        }

    }
}

// -----------------------------------------------------------------------------
//     - Pixels trait impls -
// -----------------------------------------------------------------------------

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

impl IntoIterator for Pixels {
    type Item = Pixel;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<Pixel>> for Pixels {
    fn from(p: Vec<Pixel>) -> Self {
        Pixels(p)
    }
}
