#![deny(missing_docs)]
use nalgebra::Matrix4;
use num_traits::NumCast;

use crate::{Position, Size};

/// A viewport that can be rendered into.
/// ```
/// use nightmaregl::{Size, Position, Viewport};
///
/// let viewport = Viewport::new(
///     Position::zero(),
///     Size::new(800, 600)
/// );
/// ```
#[derive(Debug)]
pub struct Viewport {
    /// The screen position of the viewport
    pub position: Position<i32>,
    pub(crate) size: Size<i32>,
    pub(crate) view: Matrix4<f32>,
    pub(crate) projection: Matrix4<f32>,
}

fn projection(size: Size<f32>) -> Matrix4<f32> {
    Matrix4::new_orthographic(
        0.0,
        size.width,
        0.0,
        size.height,
        0.0,
        -100.0,
    )
}

impl Viewport {
    /// Create a new viewport somewhere in screen space.
    pub fn new(position: impl Into<Position<i32>>, size: impl Into<Size<i32>> + Copy) -> Self {
        let size = size.into();

        Self {
            position: position.into(),
            size,
            view: Matrix4::identity(),
            projection: projection(size.cast()),
        }
    }

    /// Reszie the viewport.
    /// This will also update the projection.
    pub fn resize<T: NumCast + Copy>(&mut self, new_size: Size<T>) {
        self.size = new_size.cast();
        self.projection = projection(new_size.cast());
    }

    /// Get a reference to the size of the viewport.
    pub fn size(&self) -> &Size<i32> {
        &self.size
    }

    /// Get the middle of the viewport
    pub fn centre(&self) -> Position<i32> {
        Position::new(self.size.width / 2, self.size.height / 2)
    }
}
