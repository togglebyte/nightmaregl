#![deny(missing_docs)]
//! Default renderer.
//! Also contains [`VertexData`].
use std::ffi::CStr;
use std::ops::{Div, MulAssign};

use gl33::global_loader::*;
use gl33::*;
use nalgebra::{Matrix4, Point3, Scalar, Vector};
use num_traits::cast::NumCast;
use num_traits::{One, Zero};

use super::shaders::ShaderProgram;
use super::{GlType, Vertex, VertexPointers, QUAD};
use super::vertexpointers::{Divisor, Location, ParamCount};
use crate::context::{Context, Vao, Vbo};
use crate::sprite::{FillMode, Sprite};
use crate::{Result, Texture, Transform, Viewport};

/// Default vertex data
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexData {
    // /// The model matrix 
    // pub model: Matrix4<f32>,

    /// Texture position
    pub texture_position: (f32, f32),

    /// Texture size
    pub texture_size: (f32, f32),

    /// Tile count
    pub tile_count: (f32, f32),
}

impl VertexData {
    /// Create a new set of vertex data by combining the sprite and the transform.
    pub fn new<T: Copy + NumCast + Zero + MulAssign + Default + Scalar + Div<Output = T>>(
        sprite: &Sprite<T>,
        transform: &Transform<T>,
    ) -> Self {
        let tile_count: (f32, f32) = match sprite.fill {
            FillMode::Repeat => {
                let size = sprite.size.to_f32();
                let total_texture_size = sprite.texture_size.to_f32();
                let (texture_width, texture_height) = sprite.get_texture_size();
                let x = size.width / texture_width / total_texture_size.width;
                let y = size.height / texture_height / total_texture_size.height;
                (x, y)
            }
            FillMode::Stretch => (1.0, 1.0),
        };

        VertexData {
            // model: Self::create_model(sprite, &transform),
            texture_position: sprite.get_texture_position(),
            texture_size: sprite.get_texture_size(),
            tile_count,
        }
    }

    /// Make the vertex data relative to another transformation.
    /// This is useful when working in local space:
    ///
    /// ```
    /// use nightmaregl::{Sprite, Transform, Position, VertexData, Size};
    /// // Parent
    /// let parent_sprite = Sprite::<f32>::from_size(Size::new(32.0, 32.0));
    /// let mut parent_transform = Transform::default();
    /// parent_transform.translate_mut(Position::new(100.0, 100.0));
    ///
    /// let child_sprite = Sprite::<f32>::from_size(Size::new(32.0, 32.0));
    /// let mut child_transform = Transform::default();
    /// // Place the child 64 pixels to the right
    /// child_transform.translate_mut(Position::new(0.0, 0.0));
    /// let mut vertex_data = VertexData::new(&child_sprite, &child_transform);
    ///
    /// // Make the child relative to the parent.
    /// // By doing so, the child_sprite is placed 64 pixels to the 
    /// // right of the parent
    /// vertex_data.make_relative(&parent_transform);
    /// let pos = vertex_data.model.column(3);
    /// assert_eq!(pos[0], 100.0);
    /// assert_eq!(pos[1], 100.0);
    /// ```
    pub fn make_relative<T: Copy + NumCast + Zero + One + MulAssign + Default + Scalar + Div<Output = T>>(&mut self, relative_to: &Transform<T>) {
        let parent = relative_to.matrix();
        // self.model = parent * self.model;
    }
}

/// Default vertex pointers for [`crate::VertexData`].
/// To use different vertex data with a different layout create new `VertexPointers` with
/// a different layout.
pub fn default_vertex_pointers(context: &mut Context) -> VertexPointers {
    let mut vp = super::new_vertex_pointers();
    // vp.with_divisor(Divisor(1))

        // This used to hold the transformation matrix,
        // but since that's now a uniform we don't need this
        // .add(Position(3), ParamCount(4), GlType::Float, false)
        // .add(Position(4), ParamCount(4), GlType::Float, false)
        // .add(Position(5), ParamCount(4), GlType::Float, false)
        // .add(Position(6), ParamCount(4), GlType::Float, false)

        // .add::<[f32;2]>(Location(10), ParamCount(2), GlType::Float, false)
        // .add::<[f32;2]>(Location(11), ParamCount(2), GlType::Float, false)
        // .add::<[f32;2]>(Location(12), ParamCount(2), GlType::Float, false);

    vp
}

/// The default renderer.
///
/// ```
/// # use nightmaregl::*;
/// # fn run(mut context: Context, viewport: Viewport, sprites: Vec<Sprite<f32>>, transforms: Vec<Transform<f32>>, texture: Texture<f32>) {
/// let renderer = Renderer::default(&mut context).unwrap();
/// let vertex_data = sprites
///     .iter()
///     .zip(transforms.iter())
///     .map(|(s, t)| VertexData::new(s, t))
///     .collect::<Vec<_>>();
/// renderer.render(&texture, &vertex_data, &viewport, &mut context);
/// # }
/// ```
pub struct Renderer {
    vao: Vao,
    vbo: Vbo,
    _quad_vbo: Vbo,
    /// TODO: this shouldn't be here, I think
    pub shader_program: ShaderProgram,
    /// Multiplier for the size of a pixel.
    pub pixel_size: i32,
}

impl Renderer {
    /// Create a default renderer using the default shaders
    pub fn default(context: &mut Context) -> Result<Self> {
        panic!();
        // let vertex_pointers = default_vertex_pointers(context);
        // let shader_program = ShaderProgram::default();
        // Self::new(vertex_pointers, shader_program?)
    }
    
    /// Create a default font renderer, using the font shaders
    pub fn default_font(context: &mut Context) -> Result<Self> {
        panic!();
        // let vertex_pointers = default_vertex_pointers(context);
        // let shader_program = ShaderProgram::default_font();
        // Self::new(vertex_pointers, shader_program?)
    }

    /// Create a new renderer.
    /// A renderer needs both a vertex shader and a fragment shader.
    pub fn new(vertex_pointers: VertexPointers, shader_program: ShaderProgram) -> Result<Self> {
        panic!();
        // let mut vp = VertexPointers::new();
        // // vp
        // //     .add::<[f32; 3]>(Location(0), ParamCount(3), GlType::Float, false)
        // //     .add::<[f32; 2]>(Location(1), ParamCount(2), GlType::Float, false);

        // quad_vbo.load_data(&QUAD);

        // let inst = Self {
        //     vao,
        //     vbo,
        //     shader_program,
        //     _quad_vbo: quad_vbo,
        //     pixel_size: 1,
        // };

        // Ok(inst)
    }

    // TODO:
    // 1. Only set the VBO/VAO once

    // /// Render vertex data.
    // /// See the description of [struct::Renderer](Renderer) for an example.
    // pub fn render_instanced<U: Copy + NumCast>(
    //     &mut self,
    //     texture: &Texture<U>,
    //     vertex_data: &[T],
    //     viewport: &Viewport,
    //     context: &mut Context,
    //     transforms: &[Matrix4<f32>],
    //     chunk_size: usize,
    //     uniform: &crate::material::DefaultUniform,
    //     // material: Option<Material>
    // ) -> Result<()> {
    //     self.vbo.load_data(&vertex_data);

    //     self.shader_program.enable();

    //     context.bind_vao(&self.vao);

    //     unsafe {
    //         glViewport(
    //             viewport.position.x,
    //             viewport.position.y,
    //             viewport.size.width,
    //             viewport.size.height,
    //         );
    //     }

    //     // -----------------------------------------------------------------------------
    //     //     - Set transform / Texture offset -
    //     //     Set the transform and the texture offset in one.
    //     //     Load the uv coords from the texture.
    //     // -----------------------------------------------------------------------------
    //     texture.bind();

    //     // // Rather than using load_data, pass the data
    //     // // as a uniform
    //     // self.vbo.load_data(&vertex_data);

    //     // Clip
    //     let clip = viewport.projection * viewport.view;
    //     uniform.set_values(&self.shader_program, self.pixel_size as f32, clip);

    //     // // TODO: cache this
    //     // let clip_uniform_name = CStr::from_bytes_with_nul(b"vp\0").expect("invalid c string");
    //     // let clip_loc = self.shader_program.get_uniform_location(clip_uniform_name)?;
    //     // self.shader_program.set_uniform_matrix(clip, clip_loc);

    //     // // TODO: cache this
    //     // let pixel_scale_uniform_name =
    //     //     CStr::from_bytes_with_nul(b"pixel_scale\0").expect("invalid c string");
    //     // let pixel_scale_loc = self.shader_program.get_uniform_location(pixel_scale_uniform_name)?;
    //     // self.shader_program.set_uniform_float(self.pixel_size as f32, pixel_scale_loc);

    //     for chunk in transforms.chunks(chunk_size) {
    //         uniform.set_transform(&self.shader_program, chunk);

    //         // TODO: cache this
    //         // let transform_uniform_name = CStr::from_bytes_with_nul(b"transform\0").expect("invalid c string");
    //         // let transform_loc = self.shader_program.get_uniform_location(transform_uniform_name)?;
    //         // self.shader_program.set_uniform_matrix_array(chunk, transform_loc);

    //         unsafe {
    //             glDrawArraysInstanced(
    //                 GL_TRIANGLE_STRIP,
    //                 0,
    //                 QUAD.len() as i32,
    //                 chunk.len() as i32,
    //             )
    //         };
    //     }

    //     Ok(())
    // }
}
