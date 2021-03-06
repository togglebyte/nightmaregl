// #![deny(missing_docs)]
use std::ops::{MulAssign, Div};

use crate::{Position, Rotation, Vector};
use nalgebra::{Matrix4, Scalar, Vector as NalVector};
use num_traits::{One, Zero, NumCast};

#[derive(Debug, Copy, Clone)]
pub struct Transform<T> {
    pub translation: Position<T>,
    pub scale: Vector<T>,
    pub rotation: Rotation<T>,
}

impl<T: Copy + NumCast + Zero + One + MulAssign + Default + Scalar + Div<Output = T>> Default for Transform<T> {
    fn default() -> Self {
        Self {
            translation: Position::zero(),
            scale: Vector::new(T::one(), T::one()),
            rotation: Rotation::zero(),
        }
    }
}

impl<T: Copy + NumCast + Zero + One + MulAssign + Default + Scalar + Div<Output = T>> Transform<T> {
    pub fn new(translation: Position<T>) -> Self {
        Self {
            translation,
            scale: Vector::new(T::one(), T::one()),
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

    pub fn scale_mut(&mut self, scale: Vector<T>) {
        self.scale = scale;
    }

    pub fn transform(&self, other: &Transform<T>) -> Matrix4<f32> {
        self.matrix() * other.matrix()
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        let position = self.translation.to_f32();
        let rotation = self.rotation.to_f32();

        let rotation = NalVector::from([0.0, 0.0, rotation.radians]);
        let scale = self.scale.to_f32();

        Matrix4::new_translation(&NalVector::from([
            position.x,
            position.y,
            1.0
        ])) * Matrix4::new_rotation(rotation)
            * Matrix4::new_nonuniform_scaling(&NalVector::from([
                scale.x,
                scale.y, 
                1.0
            ]))
    }

}
