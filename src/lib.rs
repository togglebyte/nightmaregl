mod animation;
mod color;
mod context;
mod sprite;
mod viewport;
mod transform;

pub mod errors;
pub mod framebuffer;
pub mod pixels;
pub mod renderer;
pub mod texture;

#[cfg(feature = "eventloop")] pub mod events;
#[cfg(feature = "text")] pub mod text;
#[cfg(feature = "extras")] pub mod extras;

pub use errors::Result;

pub use animation::Animation;
pub use color::Color;
pub use context::Context;
pub use renderer::{default::Renderer, default::VertexData};
pub use sprite::{FillMode, Sprite};
pub use texture::Texture;
pub use viewport::{RelativeViewport, Viewport};
pub use transform::Transform;

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
pub type Point<T> = euclid::default::Point2D<T>;
pub type Rect<T> = euclid::default::Rect<T>;
pub type Rotation<T> = euclid::Angle<T>;
