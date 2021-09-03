// mod animation;
mod color;
mod context;
mod sprite;
mod viewport;
mod transform;

pub mod errors;
pub mod framebuffer;
pub mod pixels;
pub mod render;
pub mod render2d;
pub mod vertexpointers;
pub mod shaders;
pub mod texture;

#[cfg(feature = "eventloop")] pub mod events;
// #[cfg(feature = "text")] pub mod text;
#[cfg(feature = "extras")] pub mod extras;

pub use errors::Result;

// pub use animation::Animation;
pub use color::Color;
pub use color::Colour;
pub use context::{Context, Vao, Vbo};
pub use nightmare_derive::VertexData;
pub use sprite::{FillMode, Sprite};
pub use texture::Texture;
pub use transform::create_model_matrix;
pub use viewport::Viewport;

// -----------------------------------------------------------------------------
//     - Re-exports -
// -----------------------------------------------------------------------------
pub type Size = nalgebra::base::Vector2<f32>;
pub type Scale = nalgebra::base::Vector2<f32>;
pub type Position = nalgebra::base::Vector2<f32>;
pub type Vector = nalgebra::base::Vector2<f32>;
pub type Point = nalgebra::geometry::Point2<f32>;
pub type Rotation = nalgebra::geometry::Rotation2<f32>;
pub type Transform = nalgebra::geometry::Similarity2<f32>;
pub type Matrix = nalgebra::Matrix4<f32>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rect(Vector, Vector);

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self(
            Vector::new(x, y),
            Vector::new(width, height),
        )
    }
}

// pub type Size<T> = euclid::default::Size2D<T>;
// pub type Position<T> = euclid::default::Vector2D<T>;
// pub type Vector<T> = euclid::default::Vector2D<T>;
// pub type Point<T> = euclid::default::Point2D<T>;
// pub type Rect<T> = euclid::default::Rect<T>;
// pub type Rotation<T> = euclid::Angle<T>;

