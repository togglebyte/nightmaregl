// #![deny(missing_docs)]
use std::ops::{MulAssign, Div};

use nalgebra::{Point3, Vector3};

use crate::{Position, Rotation, Vector, Sprite, Matrix, Transform};

//// TODO: add a function to convert a Matrix4<T> into a Transform<T>
//#[derive(Debug, Copy, Clone)]
//pub struct Transform<T> {
//    pub translation: Position<T>,
//    pub scale: Vector<T>,
//    pub rotation: Rotation<T>,
//}

//impl<T: Copy + NumCast + Zero + One + MulAssign + Default + Scalar + Div<Output = T>> Default for Transform<T> {
//    fn default() -> Self {
//        Self {
//            translation: Position::zero(),
//            scale: Vector::new(T::one(), T::one()),
//            rotation: Rotation::zero(),
//        }
//    }
//}

//impl<T: Copy + NumCast + Zero + One + MulAssign + Default + Scalar + Div<Output = T>> Transform<T> {
//    pub fn new(translation: Position<T>) -> Self {
//        Self {
//            translation,
//            scale: Vector::new(T::one(), T::one()),
//            rotation: Rotation::zero(),
//        }
//    }

//    pub fn rotate(&self, other: Transform<T>) -> Transform<T> {
//        Transform {
//            rotation: self.rotation + other.rotation,
//            ..other
//        }
//    }

//    pub fn rotate_mut(&mut self, rot: Rotation<T>) {
//        self.rotation = rot;
//    }

//    pub fn translate(&self, other: Transform<T>) -> Transform<T> {
//        Transform {
//            translation: self.translation + other.translation,
//            ..other
//        }
//    }

//    pub fn translate_mut(&mut self, translation: Position<T>) {
//        self.translation = translation;
//    }

//    pub fn scale_mut(&mut self, scale: Vector<T>) {
//        self.scale = scale;
//    }

//    pub fn transform(&self, other: &Transform<T>) -> Matrix4<f32> {
//        self.matrix() * other.matrix()
//    }

//    pub fn matrix(&self) -> Matrix4<f32> {
//        let position = self.translation.to_f32();
//        let rotation = self.rotation.to_f32();

//        let rotation = NalVector::from([0.0, 0.0, rotation.radians]);
//        let scale = self.scale.to_f32();

//        Matrix4::new_translation(&NalVector::from([
//            position.x,
//            position.y,
//            1.0
//        ])) * Matrix4::new_rotation(rotation)
//            * Matrix4::new_nonuniform_scaling(&NalVector::from([
//                scale.x,
//                scale.y, 
//                1.0
//            ]))
//    }

//    /// Make the transformation relative to another transformation.
//    /// This is useful when working in local space:
//    ///
//    /// ```
//    /// use nightmaregl::{Sprite, Transform, Position, Size};
//    /// // Parent
//    /// let parent_sprite = Sprite::<f32>::from_size(Size::new(32.0, 32.0));
//    /// let mut parent_transform = Transform::default();
//    /// parent_transform.translate_mut(Position::new(100.0, 100.0));
//    ///
//    /// let child_sprite = Sprite::<f32>::from_size(Size::new(32.0, 32.0));
//    /// let mut child_transform = Transform::default();
//    /// // Place the child 64 pixels to the right
//    /// child_transform.translate_mut(Position::new(0.0, 0.0));
//    /// let mut vertex_data = crate_model_matrix(&child_sprite, &child_transform);
//    ///
//    /// // Make the child relative to the parent.
//    /// // By doing so, the child_sprite is placed 64 pixels to the 
//    /// // right of the parent
//    /// vertex_data.make_relative(&parent_transform);
//    /// let pos = vertex_data.model.column(3);
//    /// assert_eq!(pos[0], 100.0);
//    /// assert_eq!(pos[1], 100.0);
//    /// ```
//    pub fn relative_to(&mut self, relative_to: &Transform<T>) -> Transform<T> {
//        let parent = relative_to.matrix();
//        let new_matrix = parent * self.matrix();

//        let (translation, rotation, scale) = {
//            translation = 
//        };

//        let mut new_transform = Transform::new(translation);
//        new_transform.rotation = rotation;
//        new_transform.scale = scale;

//        new_transform
//    }

//    // pub fn make_relative_to(&mut self, relative_to: &Transform<T>) {
//    //     let parent = relative_to.matrix();
//    //     let we_want_this_well_I_want_this = parent * self.matrix();

//    //     let (translation, rotation, scale) = smush_into_parts(we_want_this_well_I_want_this);

//    //     self.translation = translation;
//    //     self.rotation = rotation;
//    //     self.scale = scale;
//    // }

//}

pub fn create_model_matrix(sprite: &Sprite, transform: &Transform) -> Matrix {
    let position = transform.isometry.translation;
    let rotation = transform.isometry.rotation.angle();
    let scale = transform.scaling();

    // let position = transform.translation.to_f32();
    // let rotation = transform.rotation.to_f32();
    let rotation = Vector3::from([0.0, 0.0, rotation]);


    let size = sprite.size;
    let anchor = sprite.anchor;
    let anchor = Point3::new(anchor.x * scale, anchor.y * scale, 0.0);

    Matrix::new_translation(&Vector3::from([
        position.x - anchor.x,
        position.y - anchor.y,
        sprite.z_index as f32,
    ])) * Matrix::new_rotation_wrt_point(rotation, anchor.into())
        * Matrix::new_nonuniform_scaling(&Vector3::from([
            size.x * scale,
            size.y * scale,
            1.0,
        ]))
}
