use std::fmt;

use bytemuck::{Pod, Zeroable};

use crate::Color;

// -----------------------------------------------------------------------------
//     - Pixel -
// -----------------------------------------------------------------------------
/// A pixel, as a set of rgba colours.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
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
//     - Greyscale pixel -
// -----------------------------------------------------------------------------
/// A grey scale pixel
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BWPixel(u8);



