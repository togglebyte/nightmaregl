// #![deny(missing_docs)]
use std::ops::AddAssign;

use nalgebra::{Vector, Point3, Matrix4};
use num_traits::{Zero, One};
use crate::{Position, Size, Rotation};
use crate::sprite::Sprite;

#[derive(Debug, Copy, Clone)]
pub struct Transform<T> {
    pub translation: Position<T>,
    pub scale: Size<T>,
    pub rotation: Rotation<T>,
}

impl<T: Zero + One + Copy + AddAssign> Default for Transform<T> {
    fn default() -> Self {
        Self {
            translation: Position::zero(),
            scale: Size::new(T::one(), T::one()),
            rotation: Rotation::zero(),
        }
    }
}

impl<T: Zero + One + Copy + AddAssign> Transform<T> {
    pub fn new() -> Self {
        Self {
            translation: Position::zero(),
            scale: Size::new(T::one(), T::one()),
            rotation: Rotation::zero(),
        }
    }

    pub fn rotate(&self, other: Transform<T>) -> Transform<T> {
        Transform {
            rotation: self.rotation + other.rotation,
            ..other
        }
    }

    pub fn rotate_mut(&mut self, rot: Rotation<T>) {
        self.rotation = rot;
    }

    pub fn translate(&self, other: Transform<T>) -> Transform<T> {
        Transform {
            translation: self.translation + other.translation,
            ..other
        }
    }

    pub fn translate_mut(&mut self, translation: Position<T>) {
        self.translation = translation;
    }

}
