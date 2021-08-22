#![deny(missing_docs)]
use std::ops::{Div, MulAssign};

use nalgebra::Scalar;
use num_traits::cast::NumCast;
use num_traits::Zero;

use crate::texture::Texture;
use crate::{Point, Position, Rect, Size};

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

/// A sprite, positioned somehwere in world space.
///
/// ```
/// # use nightmaregl::Texture;
/// use nightmaregl::{Rect, Sprite, Point, Position, Size, Transform, VertexData};
///
/// # fn run(texture: Texture<i32>) {
/// let mut sprite = Sprite::new(&texture);
/// // Sprite is "looking" at 32x32 pixels with an offset of zero
/// sprite.texture_rect = Rect::new(Point::zero(), Size::new(32, 32));
/// let transform = Transform::new(Position::new(10, 10));
/// let vertex_data = VertexData::new(&sprite, &transform);
/// # }
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Sprite<T> {
    // The texture size of the sprite
    pub(crate) texture_size: Size<T>,
    /// The size of the sprite
    pub size: Size<T>,
    /// A rectangle representing the area
    /// of a texture to render.
    pub texture_rect: Rect<T>,
    /// The anchor point of the sprite.
    /// To rotate a sprite around its centre set the anchor
    /// to be half the size of the sprite.
    pub anchor: Position<T>,
    /// The order in which this sprite appears.
    /// If a sprite has a lower `z_index` than another sprite it will
    /// be drawn above it. Note however that for alpha values to work
    /// the draw order is also important.
    pub z_index: i32,
    /// Decide whether to tile or stretch.
    pub fill: FillMode,
}

impl<T: Copy + NumCast + Zero + MulAssign + Default + Scalar + Div<Output = T>> Sprite<T> {
    /// Create a new sprite that has the size of the texture by default.
    /// To set the sprite to only show a portion of a texture set the
    /// `texture_rect` value.
    pub fn new(texture: &Texture<T>) -> Self {
        let texture_size = texture.size;
        Self::from_size(texture_size)
    }

    /// Create a new sprite using a given texture size.
    /// Avoid using this directly as using [`new`] will be less
    /// prone to human errors.
    ///
    /// However this is very useful for testing purposes.
    pub fn from_size(texture_size: Size<T>) -> Sprite<T> {
        Self {
            size: texture_size,
            texture_size,
            texture_rect: Rect::new(Point::zero(), texture_size.cast()),
            anchor: Position::zero(),
            z_index: 50,
            fill: FillMode::Stretch,
        }
    }

    // /// Create a model matrix
    // pub fn model(&self) -> Matrix4<f32> {
    //     let position = self.position.to_f32();
    //     let size = self.size.to_f32();
    //     let rotation = self.rotation.to_f32();
    //     let rotation = Vector::from([0.0, 0.0, rotation.radians]);
    //     let anchor = self.anchor.to_f32();
    //     let anchor = Point3::new(anchor.x, anchor.y, 0.0);

    //     Matrix4::new_translation(&Vector::from([
    //         position.x - anchor.x,
    //         position.y - anchor.y,
    //         self.z_index as f32,
    //     ])) * Matrix4::new_rotation_wrt_point(rotation, anchor)
    //         * Matrix4::new_nonuniform_scaling(&Vector::from([size.width, size.height, 1.0]))
    // }

//     /// Transform a sprite relative to another sprite
//     /// and produce vertex data.
//     pub fn transform(&self, sprite: &Sprite<T>) -> VertexData {
//         let vd = self.vertex_data();

//         let mut model = sprite.vertex_data().model;

//         let mut to_val = model.fixed_slice_mut::<2, 2>(0, 0);
//         let from_val = vd.model.fixed_slice::<2, 2>(0, 0);
//         to_val[0] /= from_val[0];
//         to_val[3] /= from_val[3];

//         eprintln!("model: {}", model);

//         VertexData {
//             model: vd.model * model,
//             ..vd
//         }
//     }

    // pub(crate) fn get_texture_position(&self) -> (f32, f32) {
    //     let total_tex_size = self.texture_size.to_f32();
    //     let origin = self.texture_rect.origin.to_f32();

    //     (
    //         origin.x / total_tex_size.width,
    //         origin.y / total_tex_size.height,
    //     )
    // }

    // pub(crate) fn get_texture_size(&self) -> (f32, f32) {
    //     let tex_rect_size = self.texture_rect.size.to_f32();
    //     let total_tex_size = self.texture_size.to_f32();

    //     (
    //         tex_rect_size.width / total_tex_size.width,
    //         tex_rect_size.height / total_tex_size.height,
    //     )
    // }

    // /// Convert the sprite to vertex data.
    // /// Works with the default renderer.
    // pub fn tile_count(&self) -> f32 {
    //     match self.fill {
    //         FillMode::Repeat => {
    //             let size = self.size.to_f32();
    //             let total_texture_size = self.texture_size.to_f32();
    //             let (texture_width, texture_height) = self.get_texture_size();
    //             let x = size.width / texture_width / total_texture_size.width;
    //             let y = size.height / texture_height / total_texture_size.height;
    //             (x, y)
    //         }
    //         FillMode::Stretch => (1.0, 1.0),
    //     }
    // }

    // /// Convert the sprite to vertex data.
    // /// Works with the default renderer.
    // pub fn vertex_data(&self) -> VertexData {
    //     let tile_count: (f32, f32) = match self.fill {
    //         FillMode::Repeat => {
    //             let size = self.size.to_f32();
    //             let total_texture_size = self.texture_size.to_f32();
    //             let (texture_width, texture_height) = self.get_texture_size();
    //             let x = size.width / texture_width / total_texture_size.width;
    //             let y = size.height / texture_height / total_texture_size.height;
    //             (x, y)
    //         }
    //         FillMode::Stretch => (1.0, 1.0),
    //     };

    //     VertexData {
    //         model: self.model(),
    //         texture_position: self.get_texture_position(),
    //         texture_size: self.get_texture_size(),
    //         tile_count,
    //     }
    // }
}
