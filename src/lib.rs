mod color;
mod context;
mod framebuffer;
mod pixel;
mod sprite;
mod viewport;
mod animation;
pub mod errors;
pub mod events;
pub mod renderer;
pub mod text;
pub mod texture;

pub use errors::Result;

pub use context::Context;
pub use framebuffer::{Framebuffer, FramebufferTarget};
pub use sprite::{VertexData, Sprite, FillMode};
pub use texture::Texture;
pub use viewport::Viewport;
pub use color::Color;
pub use renderer::Renderer;
pub use pixel::{Pixel, Pixels};
pub use animation::Animation;

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
