use std::ops::MulAssign;

use nalgebra::{Matrix4, Vector};
use num_traits::cast::NumCast;
use num_traits::Zero;

use crate::{Position, Rotation, Size};

pub type VertexData = (
    // Model
    Matrix4<f32>, 
    // Offset
    (f32, f32), 
    // Scale
    (f32, f32)
);

// -----------------------------------------------------------------------------
//     - Sprite -
// -----------------------------------------------------------------------------
/// A sprite, positioned in world space.
#[derive(Debug, Copy, Clone)]
pub struct Sprite<T> {
    pub size: Size<T>,
    pub texture_size: Size<T>,
    pub position: Position<T>,
    pub rotation: Rotation<T>,
    pub texture_offset: Position<T>,
    pub z_index: T,
}

impl<T: Copy + NumCast + Zero + MulAssign> Sprite<T> {
    /// Create a new sprite that has the size of the texture by default.
    /// To set the sprite to only show a portion of a texture set the 
    /// `texture_offset` value.
    pub fn new(texture_size: impl Into<Size<T>>) -> Self {
        let texture_size = texture_size.into();

        Self {
            size: texture_size,
            texture_size: texture_size,
            position: Position::zero(),
            rotation: Rotation::zero(),
            texture_offset: Position::zero(),
            z_index: T::zero(),
        }
    }

    /// Model matrix
    pub fn model(&self) -> Matrix4<f32> {
        let position = self.position.to_f32();
        let size = self.size.to_f32();
        let rotation = self.rotation.to_f32();
        Matrix4::new_translation(&Vector::from([
            position.x,
            position.y,
            self.z_index.to_f32().unwrap(),
        ])) * Matrix4::new_rotation(Vector::from([0., 0., rotation.radians]))
            * Matrix4::new_nonuniform_scaling(&Vector::from([size.width, size.height, 1.0]))
    }

    fn get_texture_offset(&self) -> (f32, f32) {
        let offset = self.texture_offset.to_f32();
        let size = self.texture_size.to_f32();
        let x = offset.x / size.width;
        let y = offset.y / size.height;
        (x, y)
    }

    fn get_texture_scale(&self) -> (f32, f32) {
        let size = self.size.to_f32();
        let texture_size = self.texture_size.to_f32();
        let x = size.width / texture_size.width;
        let y = size.height / texture_size.height;
        (x, y)
    }

    /// Convert the sprite to vertex data.
    /// Works with the default renderer.
    pub fn vertex_data(&self) -> VertexData {
        (
            self.model(),
            self.get_texture_offset(),
            self.get_texture_scale(),
        )
    }
}
