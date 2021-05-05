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
        -10000.0,
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

    /// Swap the Y axis on the projection matrix.
    /// This is useful if rendering to a framebuffer
    /// as the y axis will be flipped by default.
    pub fn swap_y(&mut self) {
        let size = self.size.cast();
        let matrix = Matrix4::new_orthographic(
            0.0,
            size.width,
            size.height,
            0.0,
            0.0,
            -10000.0,
        );

        self.projection = matrix;
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

    /// Create a sub viewport based on this viewport.
    /// The position is the offset from the bottom left corner of the viewport.
    /// The size is the padding omghelpwhat
    ///
    /// ```
    /// use nightmaregl::{Size, Position, Viewport};
    /// let mut main_vp = Viewport::new(Position::new(10, 10), Size::new(100, 100));
    /// let mut sub = main_vp.sub_viewport(Position::new(5, 5), Size::new(10, 10));
    /// assert_eq!(sub.viewport().position, Position::new(10 + 5, 10 + 5));
    /// assert_eq!(*sub.viewport().size(), Size::new(100 - 10, 100 - 10));
    ///
    /// // Resize the main viewport
    /// main_vp.resize(Size::new(50, 50));
    /// sub.resize(&main_vp);
    /// assert_eq!(*sub.viewport().size(), Size::new(50 - 10, 50 - 10));
    /// ```
    pub fn sub_viewport(
        &self,
        bottom_left: Position<i32>,
        top_right: Position<i32>
    ) -> RelativeViewport {
        RelativeViewport::new(bottom_left, top_right, &self)
    }
}

// -----------------------------------------------------------------------------
//     - Relative viewport -
// -----------------------------------------------------------------------------
/// This viewport is relative to another viewport.
#[derive(Debug)]
pub struct RelativeViewport {
    inner: Viewport,
    bottom_left: Position<i32>,
    top_right: Position<i32>,
}

impl RelativeViewport {
    fn new(bottom_left: Position<i32>, top_right: Position<i32>, relative_to: &Viewport) -> Self {
        let position = relative_to.position + bottom_left;

        let size = Size::new(
            relative_to.size.width - top_right.x,
            relative_to.size.height - top_right.y,
        );

        let inner = Viewport::new(
            position,
            size,
        );

        Self {
            inner,
            bottom_left,
            top_right,
        }
    }

    /// Resize the viewport based on the relative viewport.
    pub fn resize(&mut self, relative_to: &Viewport) {
        self.inner.resize(relative_to.size - Size::new(self.top_right.x, self.top_right.y));
    }

    pub fn viewport(&self) -> &Viewport {
        &self.inner
    }
}
