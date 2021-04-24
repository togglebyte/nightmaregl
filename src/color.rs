#![deny(missing_docs)]
use crate::pixels::Pixel;

#[derive(Debug, Copy, Clone)]
/// A colour with values ranging from 0.0 to 1.0
pub struct Color {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
    /// Alpha
    pub a: f32,
}

impl Color {
    /// All white
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    /// All grey
    pub fn grey() -> Self {
        Self {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        }
    }

    /// All black
    pub fn black() -> Self {
        Self::default()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl From<Pixel> for Color {
    fn from(p: Pixel) -> Self {
        Self {
            r: (p.r as f32 / 255.0),
            g: (p.g as f32 / 255.0),
            b: (p.b as f32 / 255.0),
            a: (p.a as f32 / 255.0),
        }
    }
}
