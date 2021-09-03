#![deny(missing_docs)]
use std::ops::{Div, MulAssign};

use crate::texture::Texture;
use crate::{Point, Position, Size, Vector, Matrix, Rect};

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
pub struct Sprite {
    // The texture size of the sprite
    pub(crate) texture_size: Size,
    /// The size of the sprite
    pub size: Size,
    /// A rectangle representing the area
    /// of a texture to render.
    pub texture_rect: Rect,
    /// The anchor point of the sprite.
    /// To rotate a sprite around its centre set the anchor
    /// to be half the size of the sprite.
    pub anchor: Position,
    /// The order in which this sprite appears.
    /// If a sprite has a lower `z_index` than another sprite it will
    /// be drawn above it. Note however that for alpha values to work
    /// the draw order is also important.
    pub z_index: i32,
    /// Decide whether to tile or stretch.
    pub fill: FillMode,
}

impl Sprite {
    /// Create a new sprite that has the size of the texture by default.
    /// To set the sprite to only show a portion of a texture set the
    /// `texture_rect` value.
    pub fn new(texture: &Texture) -> Self {
        let texture_size = texture.size;
        Self::from_size(texture_size)
    }

    /// Create a new sprite using a given texture size.
    /// Avoid using this directly as using [`new`] will be less
    /// prone to human errors.
    ///
    /// However this is very useful for testing purposes.
    pub fn from_size(texture_size: Size) -> Sprite {
        Self {
            size: texture_size,
            texture_size,
            texture_rect: Rect::new(0.0, 0.0, texture_size.x, texture_size.y),
            anchor: Position::zeros(),
            z_index: 50,
            fill: FillMode::Stretch,
        }
    }

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
