#![deny(missing_docs)]
use gl33::global_loader::*;

use crate::{Position, Size, Matrix};

/// A viewport that can be rendered into.
/// ```
/// use nightmaregl::{Size, Position, Viewport};
///
/// let viewport = Viewport::new(
///     Position::zero(),
///     Size::new(800, 600)
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Viewport {
    /// The screen position of the viewport
    pub position: Position,
    pub(crate) size: Size,
    pub(crate) view: Matrix,
    pub(crate) projection: Matrix,
}

fn projection(size: Size) -> Matrix {
    Matrix::new_orthographic(
        0.0,
        size.x,
        0.0,
        size.y,
        0.0,
        -10000.0,
    )
}

impl Viewport {
    /// Create a new viewport somewhere in screen space.
    pub fn new(position: impl Into<Position>, size: impl Into<Size> + Copy) -> Self {
        let size = size.into();

        Self {
            position: position.into(),
            size,
            view: Matrix::identity(),
            projection: projection(size.cast()),
        }
    }

    /// Swap the Y axis on the projection matrix.
    /// This is useful if rendering to a framebuffer
    /// as the y axis will be flipped by default.
    pub fn swap_y(&mut self) {
        let size = self.size.cast();
        let matrix = Matrix::new_orthographic(
            0.0,
            size.x,
            size.y,
            0.0,
            0.0,
            -10000.0,
        );

        self.projection = matrix;
    }

    /// Reszie the viewport.
    /// This will also update the projection.
    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size.cast();
        self.projection = projection(new_size.cast());
    }

    /// Get a reference to the size of the viewport.
    pub fn size(&self) -> &Size {
        &self.size
    }

    /// Get the middle of the viewport
    pub fn centre(&self) -> Position {
        Position::new(self.size.x / 2.0, self.size.y / 2.0)
    }

    /// Set the OpenGL viewport
    pub fn set_gl_viewport(&self) {
        unsafe {
            glViewport(
                self.position.x as i32,
                self.position.y as i32,
                self.size.x as i32,
                self.size.y as i32,
            );
        }
    }

    /// Get the view projection by multiplying the projection matrix
    /// with the view matrix.
    pub fn view_projection(&self) -> Matrix {
        self.projection * self.view
    }
}
