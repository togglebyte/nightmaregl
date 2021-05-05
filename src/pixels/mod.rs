#![deny(missing_docs)]
//! Represent `Pixel`s as bytes.
//!
//! ```
//! use nightmaregl::pixels::{Pixel, Pixels};
//! use nightmaregl::Size;
//! let green = Pixel { g: 255, ..Default::default() };
//! let red = Pixel { r: 255, ..Default::default() };
//! let pixels = Pixels::new([green, red], Size::new(2, 1));
//!
//! let bytes = pixels.as_bytes();
//! ```
use std::ops::{Deref, DerefMut, Index, IndexMut};

use bytemuck::Pod;

use crate::{Position, Size};

mod region;
mod pixel;

pub use pixel::{Pixel, BWPixel};
pub use region::{Region, RegionMut};

// -----------------------------------------------------------------------------
//     - Pixel container -
// -----------------------------------------------------------------------------
/// Holds a bunch of pixels.
/// This is useful because of the `as_bytes` method which conveniently
/// represents all pixels as a byte slice.
///
/// ```
/// use nightmaregl::pixels::{Pixel, Pixels};
/// # use nightmaregl::Size;
/// # fn run() {
/// // Pixels from a collection of pixels
/// let pixels = Pixels::new(vec![Pixel::default()], Size::new(1, 1));
///
/// // Pixels from a single pixel copied to fill the entire
/// // pixel buffer
/// let pixels = Pixels::from_pixel(Pixel::default(), Size::new(20, 20));
/// # }
/// ```
#[derive(Debug)]
// pub struct Pixels(Vec<Pixel>);
pub struct Pixels<T: Pod> {
    inner: Vec<T>,
    size: Size<usize>,
}

impl<T: Pod> Pixels<T> {
    /// Create new `Pixels`.
    pub fn new(inner: impl Into<Vec<T>>, size: Size<usize>) -> Self {
        let inner = inner.into();
        debug_assert!(inner.len() == size.width * size.height);

        Self { 
           inner: inner.into(),
           size,
        }
    }

    // /// Allocate a collection of pixels.
    // /// Note that this will not fill the buffer with values,
    // /// so the length of this buffer is really 0.
    // /// This is bad if this is passed to `write_region` of a `Texture` as
    // /// `glTexSubImage2D` is expecting to get width * height number of pixels,
    // /// and if this isn't send, then the gpu will have to work with rubbish data.
    // pub(crate) fn from_size_unchecked(size: Size<usize>) -> Self {
    //     let cap = size.width * size.height;
    //     Self {
    //         inner: Vec::with_capacity(cap),
    //         size,
    //     }
    // }

    /// Repeat the pixel width * height times.
    pub fn from_pixel(pixel: T, size: Size<usize>) -> Self {
        let cap = size.width * size.height;
        Self { 
            inner: vec![pixel; cap],
            size,
        }
    }

    /// Interpret the pixels as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.inner)
    }

    /// Get the size of the pixel buffer
    /// as a width and a height.
    pub fn size(&self) -> Size<usize> {
        self.size
    }

    /// Get an iterator over a region.
    pub fn region(
        &self,
        position: Position<usize>,
        size: Size<usize>,
    ) -> Region<T> {
        debug_assert!(self.size.width >= size.width + position.x);
        debug_assert!(self.size.height >= size.height + position.y);

        let region = self
            .chunks_exact(self.size.width)
            .skip(position.y)
            .take(size.height)
            .map(|c| &c[position.x..size.width + position.x])
            .collect::<Vec<_>>();

        Region { inner: region }
    }

    /// Get an iterator over a region
    pub fn region_mut(
        &mut self,
        position: Position<usize>,
        size: Size<usize>,
    ) -> RegionMut<T> {
        debug_assert!(self.size.width >= size.width + position.x);
        debug_assert!(self.size.height >= size.height + position.y);

        let width = self.size.width;
        let region = self
            .chunks_exact_mut(width)
            .skip(position.y)
            .take(size.height)
            .map(|c| &mut c[position.x..size.width + position.x])
            .collect::<Vec<_>>();

        RegionMut { inner: region }
    }

    /// Write a region of pixels
    pub fn write_region(
        &mut self,
        position: Position<usize>,
        region: Region<T>,
    ) {

        for (i, row) in region.rows().enumerate() {
            let y = (position.y + i) * self.size.width;
            let index = y + position.x;
            let dest = &mut self.inner[index..index + row.len()];
            dest.copy_from_slice(row);
        }

    }

    /// Insert a pixel at a given location.
    pub fn insert_pixel(&mut self, pixel: T, pos: Position<usize>) {
        debug_assert!(pos.x <= self.size.width);
        debug_assert!(pos.y <= self.size.height);
        let index = pos.y * self.size.width + pos.x;
        self.inner[index] = pixel;
    }
}

// -----------------------------------------------------------------------------
//     - Pixels trait impls -
// -----------------------------------------------------------------------------

impl<T: Pod> Index<usize> for Pixels<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T: Pod> IndexMut<usize> for Pixels<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T: Pod> Deref for Pixels<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Pod> DerefMut for Pixels<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Pod> IntoIterator for Pixels<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
