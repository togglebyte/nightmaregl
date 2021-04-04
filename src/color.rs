#[derive(Debug, Copy, Clone)]
/// A colour with values ranging from 0.0 to 1.0
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub fn grey() -> Self {
        Self {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        }
    }
}

impl Default for Color  {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}
