use gl33::*;
use gl33::global_loader::*;

mod animation;
mod color;
mod context;
mod sprite;
mod viewport;
mod transform;

pub mod errors;
pub mod framebuffer;
pub mod pixels;
pub mod render;
pub mod vertexpointers;
pub mod shaders;
pub mod texture;

#[cfg(feature = "eventloop")] pub mod events;
// #[cfg(feature = "text")] pub mod text;
#[cfg(feature = "extras")] pub mod extras;
#[cfg(feature = "derive")] pub use nightmare_derive::VertexData;

pub use errors::Result;

pub use animation::Animation;
pub use color::Color;
pub use color::Colour;
pub use context::{Context, Vao, Vbo};
pub use sprite::{FillMode, Sprite};
pub use texture::Texture;
pub use viewport::Viewport;
pub use transform::{Transform, create_model_matrix};

// -----------------------------------------------------------------------------
//     - Vertex -
// -----------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pos: [f32; 3],
    uv_coords: [f32; 2],
}

// -----------------------------------------------------------------------------
//     - World -
// -----------------------------------------------------------------------------
pub type Size<T> = euclid::default::Size2D<T>;
pub type Position<T> = euclid::default::Vector2D<T>;
pub type Vector<T> = euclid::default::Vector2D<T>;
pub type Point<T> = euclid::default::Point2D<T>;
pub type Rect<T> = euclid::default::Rect<T>;
pub type Rotation<T> = euclid::Angle<T>;

