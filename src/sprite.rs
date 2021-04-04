#![deny(missing_docs)]
use std::ops::MulAssign;

use nalgebra::{Matrix4, Vector, Point3, Scalar};
use num_traits::cast::NumCast;
use num_traits::Zero;

use crate::{Position, Rotation, Size};

// /// Default vertex data
// pub type VertexData = (
//     // Model
//     Matrix4<f32>, 
//     // Offset
//     (f32, f32), 
//     // Scale
//     (f32, f32)
// );

/// Default vertex data
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexData {
    model: Matrix4<f32>, 
    offset: (f32, f32), 
    scale: (f32, f32)
}

// -----------------------------------------------------------------------------
//     - Sprite -
// -----------------------------------------------------------------------------
/// A sprite, positioned somehwere in world space. 
#[derive(Debug, Copy, Clone)]
pub struct Sprite<T> {
    /// The size of the sprite
    pub size: Size<T>,
    /// Texture offset.
    /// Used with the texture size to select a region on the
    /// texture to render.
    pub texture_offset: Position<T>,
    /// The texture size of the sprite
    pub texture_size: Size<T>,
    /// The sprites position in the world
    pub position: Position<T>,
    /// The sprites current rotation
    pub rotation: Rotation<T>,
    /// The anchor point of the sprite.
    /// To rotate a sprite around its centre set the anchor 
    /// to be half the size of the sprite.
    pub anchor: Position<T>,
    /// The order in which this sprite appears.
    /// If a sprite has a lower `z_index` than another sprite it will
    /// be drawn above it. Note however that for alpha values to work
    /// the draw order is also important.
    pub z_index: T,
}

impl<T: Copy + NumCast + Zero + MulAssign + Default + Scalar> Sprite<T> {
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
            anchor: Position::zero(),
            z_index: T::zero(),
        }
    }

    /// Model matrix
    pub fn model(&self) -> Matrix4<f32> {
        let position = self.position.to_f32();
        let size = self.size.to_f32();
        let rotation = self.rotation.to_f32();
        let rotation = Vector::from([0.0, 0.0, rotation.radians]);
        let anchor = self.anchor.to_f32();
        let anchor = Point3::new(anchor.x, anchor.y, 0.0);
        Matrix4::new_translation(&Vector::from([
            position.x,
            position.y,
            self.z_index.to_f32().unwrap(),
        ])) 
            * Matrix4::new_rotation_wrt_point(rotation, anchor)
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
        VertexData {
            model: self.model(),
            offset: self.get_texture_offset(),
            scale: self.get_texture_scale(),
        }
    }
}
