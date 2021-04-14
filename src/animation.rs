#![deny(missing_docs)]
use num_traits::cast::NumCast;

use crate::sprite::Sprite;
use crate::Point;

/// Represent a sprite as an animation.
///
/// To make the animation loop set the `should_loop` variable;
///
/// ```
/// use nightmaregl::{Sprite, Animation, Point, Size};
/// let sprite = Sprite::from_size(Size::new(32, 64));
/// let mut animation = Animation::new(sprite, 1, 3, 32);
/// animation.should_loop = false;
/// animation.fps = 1.0;
///
/// // first frame is at 0, 0
/// assert_eq!(animation.sprite.texture_rect.origin, Point::zero());
/// assert_eq!(animation.current_frame(), 0);
///
/// // Second frame is at 32, 0
/// animation.update(1.0);
/// assert_eq!(animation.sprite.texture_rect.origin, Point::new(32, 0));
/// assert_eq!(animation.current_frame(), 1);
/// 
/// // Third frame at 64, 0
/// animation.update(1.0);
/// assert_eq!(animation.sprite.texture_rect.origin, Point::new(64, 0));
/// assert_eq!(animation.current_frame(), 2);
/// ```
pub struct Animation<T> {
    cols: u16,
    stride: u16,
    current_frame: u16,
    max_frame: u16,
    /// Should this animation loop?
    pub should_loop: bool,
    /// The sprite the animation is acting upon
    pub sprite: Sprite<T>,
    /// Number of frames per second
    pub fps: f32,
    elapsed: f32,
}

impl<T: Copy + NumCast> Animation<T> {
    /// Create a new animations, where `stride` is the distance between
    /// frames. This means that a sprite sheet has to contain frames that are all
    /// of the same size.
    pub fn new(sprite: Sprite<T>, rows: u16, cols: u16, stride: u16) -> Self {
        let max_frame = rows * cols;

        Self {
            cols,
            stride,
            current_frame: 0,
            should_loop: false,
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

        if self.elapsed > sec {
            self.elapsed -= sec;
            self.next();
        }
    }

    /// Get the current frame, starting from zero.
    pub fn current_frame(&self) -> u16 {
        self.current_frame
    }

    fn next(&mut self) {
        if self.current_frame == self.max_frame - 1 {
            match self.should_loop {
                true => self.current_frame = 0,
                false => return,
            }
        } else {
            self.current_frame += 1;
        }

        let x = self.current_frame % self.cols;
        let y = self.current_frame / self.cols;

        let offset = Point::new(x * self.stride, y * self.stride).cast();
        self.sprite.texture_rect.origin = offset;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Size, Sprite};
    use crate::texture::test_texture;

    fn make_sprite() -> Sprite<u16> {
        let texture = test_texture(Size::new(32 * 2, 32 * 2));
        let mut sprite = Sprite::new(&texture);
        sprite.size = Size::new(32, 32);
        sprite
    }

    #[test]
    fn test_looping_animation_offset() {
        let stride = 32;
        let sprite = make_sprite();
        let mut animation = Animation::new(sprite, 2, 2, stride);
        animation.should_loop = true;

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
