#![deny(missing_docs)]
use std::ops::{Div, MulAssign};

use nalgebra::{Matrix4, Point3, Scalar, Vector};
use num_traits::cast::NumCast;
use num_traits::Zero;

use crate::texture::Texture;
use crate::{Point, Position, Rect, Rotation, Size};

/// Default vertex data
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexData {
    /// The model matrix
    pub model: Matrix4<f32>,

    /// Texture position
    pub texture_position: (f32, f32),

    /// Texture size
    pub texture_size: (f32, f32),

    /// Tile count
    pub tile_count: (f32, f32),
}

/// Tiling mode. Either stretch or tiling
#[derive(Debug, Copy, Clone)]
pub enum FillMode {
    /// Stretch the texture to cover the entire
    /// sprite size.
    Stretch,

    /// Repeat a portion of the texture over
    /// the entire sprite.
    Repeat,
}

// -----------------------------------------------------------------------------
//     - Sprite -
// -----------------------------------------------------------------------------
/// A sprite, positioned somehwere in world space.
#[derive(Debug, Copy, Clone)]
pub struct Sprite<T> {
    // The texture size of the sprite
    texture_size: Size<T>,
    /// The size of the sprite
    pub size: Size<T>,
    /// A rectangle representing the area
    /// of a texture to render.
    pub texture_rect: Rect<T>,
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
    /// Decide whether to tile or stretch.
    pub fill: FillMode,
}

impl<T: Copy + NumCast + Zero + MulAssign + Default + Scalar + Div<Output = T>> Sprite<T> {
    /// Create a new sprite that has the size of the texture by default.
    /// To set the sprite to only show a portion of a texture set the
    /// `texture_rect` value.
    pub fn new(texture: &Texture<T>) -> Self {
        let texture_size = texture.size;

        Self {
            size: texture_size,
            texture_size: texture_size,
            position: Position::zero(),
            rotation: Rotation::zero(),
            texture_rect: Rect::new(Point::zero(), texture_size.cast()),
            anchor: Position::zero(),
            z_index: T::zero(),
            fill: FillMode::Stretch,
        }
    }

    /// Create a model matrix
    pub fn model(&self) -> Matrix4<f32> {
        self.model_scaled(1.0)
    }

    /// Crate a scaled model matrix
    pub fn model_scaled(&self, scale: f32) -> Matrix4<f32> {
        let scale = 1.0 / scale;
        let position = self.position.to_f32() * scale;
        let size = self.size.to_f32();
        let rotation = self.rotation.to_f32();
        let rotation = Vector::from([0.0, 0.0, rotation.radians]);
        let anchor = self.anchor.to_f32();
        let anchor = Point3::new(anchor.x, anchor.y, 0.0);
        Matrix4::new_translation(&Vector::from([
            position.x + anchor.x,
            position.y + anchor.y,
            self.z_index.to_f32().unwrap(),
        ])) * Matrix4::new_rotation_wrt_point(rotation, anchor)
            * Matrix4::new_nonuniform_scaling(&Vector::from([size.width, size.height, 1.0]))
    }

    fn get_texture_position(&self) -> (f32, f32) {
        let total_tex_size = self.texture_size.to_f32();
        let origin = self.texture_rect.origin.to_f32();

        (
            origin.x / total_tex_size.width,
            origin.y / total_tex_size.height,
        )
    }

    fn get_texture_size(&self) -> (f32, f32) {
        let tex_rect_size = self.texture_rect.size.to_f32();
        let total_tex_size = self.texture_size.to_f32();

        (
            tex_rect_size.width / total_tex_size.width,
            tex_rect_size.height / total_tex_size.height,
        )
    }

    /// Convert the sprite to vertex data.
    /// Works with the default renderer.
    pub fn vertex_data(&self) -> VertexData {
        self.vertex_data_scaled(1.0)
    }

    /// Convert the sprite to vertex data.
    /// Works with the default renderer.
    pub fn vertex_data_scaled(&self, scale: f32) -> VertexData {
        let tile_count: (f32, f32) = match self.fill {
            FillMode::Repeat => {
                let size = self.size.to_f32();
                let total_texture_size = self.texture_size.to_f32();
                let (texture_width, texture_height) = self.get_texture_size();
                let x = size.width / texture_width / total_texture_size.width;
                let y = size.height / texture_height / total_texture_size.height;
                (x, y)
            }
            FillMode::Stretch => (1.0, 1.0),
        };

        VertexData {
            model: self.model_scaled(scale),
            texture_position: self.get_texture_position(),
            texture_size: self.get_texture_size(),
            tile_count,
        }
    }
}
