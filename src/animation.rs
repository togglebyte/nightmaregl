#![deny(missing_docs)]
use std::ops::{Div, MulAssign};

use nalgebra::Scalar;
use num_traits::cast::NumCast;
use num_traits::Zero;

use crate::sprite::Sprite;
use crate::texture::Texture;
use crate::{Point, Rect, Size, VertexData};

/// Represent a sprite as an animation.
///
/// To make the animation loop set the `repeat` variable;
///
/// ```
/// use nightmaregl::{Sprite, Animation, Point, Size};
/// let sprite = Sprite::from_size(Size::new(32, 64));
/// let mut animation = Animation::from_sprite(sprite, 1, 3, 32, 32);
/// animation.repeat = false;
/// animation.fps = 1.0;
///
/// // first frame is at 0, 0
/// assert_eq!(animation.sprite.texture_rect.origin, Point::zero());
/// assert_eq!(animation.current_frame(), 0);
///
/// // Second frame is at 32, 0
/// animation.update(1.0);
/// eprintln!("origin be all like: {:?}", animation.sprite.texture_rect.origin);
/// assert_eq!(animation.sprite.texture_rect.origin, Point::new(32, 0));
/// assert_eq!(animation.current_frame(), 1);
///
/// // Third frame at 64, 0
/// animation.update(1.0);
/// assert_eq!(animation.sprite.texture_rect.origin, Point::new(64, 0));
/// assert_eq!(animation.current_frame(), 2);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Animation<T> {
    cols: u16,
    stride_w: u16,
    stride_h: u16,
    current_frame: u16,
    max_frame: u16,
    /// Should this animation repeat forever?
    pub repeat: bool,
    /// The sprite the animation is acting upon
    pub sprite: Sprite<T>,
    /// Number of frames per second
    pub fps: f32,
    elapsed: f32,
}

impl<T: Copy + NumCast + Zero + MulAssign + Default + Scalar + Div<Output = T>> Animation<T> {
    /// Document me not for I shall not be here long...
    pub fn from_texture(
        texture: &Texture<T>,
        rows: u16,
        cols: u16,
        stride_w: u16,
        stride_h: u16,
    ) -> Self {
        let mut sprite = Sprite::new(texture);
        sprite.texture_rect = Rect::new(Point::zero(), Size::new(stride_w, stride_h).cast());
        sprite.size = Size::new(stride_w, stride_h).cast();
        Self::from_sprite(sprite, rows, cols, stride_w, stride_h)
    }

    /// Create a new animations, where `stride` is the distance between
    /// frames. This means that a sprite sheet has to contain frames that are all
    /// of the same size.
    pub fn from_sprite(
        sprite: Sprite<T>,
        rows: u16,
        cols: u16,
        stride_w: u16,
        stride_h: u16,
    ) -> Self {
        let max_frame = rows * cols;

        Self {
            cols,
            stride_w,
            stride_h,
            current_frame: 0,
            repeat: false,
            max_frame,
            sprite,
            fps: 10.0,
            elapsed: 0.,
        }
    }

    /// Update the time of the animation.
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        let sec = 1.0 / self.fps;

        if self.elapsed >= sec {
            self.elapsed -= sec;
            self.next();
        }
    }

    /// Get the current frame, starting from zero.
    pub fn current_frame(&self) -> u16 {
        self.current_frame
    }

    /// Get the vertex data from the underlying sprite.
    pub fn vertex_data(&self) -> VertexData {
        self.sprite.vertex_data()
    }

    fn next(&mut self) {
        if self.current_frame == self.max_frame - 1 {
            match self.repeat {
                true => self.current_frame = 0,
                false => return,
            }
        } else {
            self.current_frame += 1;
        }

        let x = self.current_frame % self.cols;
        let y = self.current_frame / self.cols;

        let offset = Point::new(x * self.stride_w, y * self.stride_h).cast();
        self.sprite.texture_rect.origin = offset;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::texture::test_texture;
    use crate::{Size, Sprite};

    fn make_sprite() -> Sprite<u16> {
        let mut sprite = Sprite::from_size(Size::new(32 * 2, 32 * 2));
        sprite.size = Size::new(32, 32);
        sprite.texture_rect = Rect::new(Point::zero(), sprite.size);
        sprite
    }

    #[test]
    fn test_looping_animation_offset() {
        let stride = 32;
        let sprite = make_sprite();
        let mut animation = Animation::from_sprite(sprite, 2, 2, stride, stride);
        animation.repeat = true;

        // Second frame
        animation.next();
        let actual = animation.sprite.texture_rect.origin;
        let expected = Point::new(stride, 0);
        assert_eq!(expected, actual);

        // Third frame
        animation.next();
        let actual = animation.sprite.texture_rect.origin;
        let expected = Point::new(0, stride);
        assert_eq!(expected, actual);

        // Fourth frame
        animation.next();
        let actual = animation.sprite.texture_rect.origin;
        let expected = Point::new(stride, stride);
        assert_eq!(expected, actual);

        // First frame: the offset stays the same
        animation.next();
        let actual = animation.sprite.texture_rect.origin;
        let expected = Point::zero();
        assert_eq!(expected, actual);
    }

    // #[test]
    // fn test_animation_ends() {
    //     let stride = 32;
    //     let sprite = make_sprite();
    //     let mut animation = Animation::new(sprite, 2, 2, stride);

    //     for _ in 0..2*2 {
    //         animation.next();
    //     }

    //     let expected = Position::new(32, 32);
    //     let actual = animation.sprite.texture_offset;
    //     assert_eq!(expected, actual);
    // }
}
