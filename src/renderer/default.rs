use std::ffi::CStr;
use std::ops::{Div, MulAssign};

use nalgebra::{Point3, Vector, Matrix4, Scalar};
use gl33::global_loader::*;
use gl33::*;
use num_traits::cast::NumCast;
use num_traits::Zero;

use super::shaders::ShaderProgram;
use super::{GlType, Vbo, Vertex, VertexPointers, QUAD};
use crate::context::{Context, Vao};
use crate::{Result, Viewport, Texture, Transform};
use crate::sprite::{Sprite, FillMode};

/// Default vertex data
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VertexData {
    /// The model matrix
    pub model: Matrix4<f32>,

    /// Texture position
    pub texture_position: (f32, f32),

    /// Texture size
    pub texture_size: (f32, f32),

    /// Tile count
    pub tile_count: (f32, f32),
}

impl VertexData {
    pub fn new<T: Copy + NumCast + Zero + MulAssign + Default + Scalar + Div<Output = T>>(sprite: &Sprite<T>, transform: &Transform<T>) -> Self {
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
            model: Self::model(sprite, &transform),
            texture_position: sprite.get_texture_position(),
            texture_size: sprite.get_texture_size(),
            tile_count,
        }
    }

    fn model<T: Copy + NumCast + Zero + MulAssign + Default + Scalar + Div<Output = T>>(sprite: &Sprite<T>, transform: &Transform<T>) -> Matrix4<f32> {
        let position = transform.translation.to_f32();
        let rotation = transform.rotation.to_f32();
        let rotation = Vector::from([0.0, 0.0, rotation.radians]);
    
        let size = sprite.size.to_f32();
        let anchor = sprite.anchor.to_f32();
        let anchor = Point3::new(anchor.x, anchor.y, 0.0);

        let scale = transform.scale.to_f32();

        Matrix4::new_translation(&Vector::from([
            position.x - anchor.x,
            position.y - anchor.y,
            sprite.z_index as f32,
        ])) * Matrix4::new_rotation_wrt_point(rotation, anchor)
            * Matrix4::new_nonuniform_scaling(&Vector::from([
                size.width * scale.width,
                size.height * scale.height, 
                1.0
            ]))
    }

}

/// Default vertex pointers for [`crate::VertexData`].
/// To use different vertex data with a different layout create new `VertexPointers` with
/// a different layout.
pub fn default_vertex_pointers<T>(context: &mut Context) -> VertexPointers<T> {
    super::new_vertex_pointers(context)
        .with_divisor(1)
        .add(3, 4, GlType::Float, false)
        .add(4, 4, GlType::Float, false)
        .add(5, 4, GlType::Float, false)
        .add(6, 4, GlType::Float, false)
        .add(10, 2, GlType::Float, false)
        .add(11, 2, GlType::Float, false)
        .add(12, 2, GlType::Float, false)
}

/// The default renderer.
///
/// ```
/// # use nightmaregl::*;
/// # fn run(mut context: Context, viewport: Viewport, sprites: Vec<Sprite<f32>>, texture: Texture<f32>) {
/// let renderer = Renderer::default(&mut context).unwrap();
/// let vertex_data = sprites
///     .iter()
///     .map(Sprite::vertex_data)
///     .collect::<Vec<_>>();
/// renderer.render(&texture, &vertex_data, &viewport, &mut context);
/// # }
/// ```
pub struct Renderer<T> {
    vao: Vao,
    vbo: Vbo<T>,
    _quad_vbo: Vbo<Vertex>,
    shader_program: ShaderProgram,
    pub pixel_size: i32,
}

impl<T: std::fmt::Debug> Renderer<T> {
    pub fn default(context: &mut Context) -> Result<Self> {
        let vertex_pointers = default_vertex_pointers(context);
        let shader_program = ShaderProgram::default();
        Self::new(vertex_pointers, shader_program?)
    }

    pub fn default_font(context: &mut Context) -> Result<Self> {
        let vertex_pointers = default_vertex_pointers(context);
        let shader_program = ShaderProgram::default_font();
        Self::new(vertex_pointers, shader_program?)
    }

    /// Create a new renderer.
    /// A renderer needs both a vertex shader and a fragment shader.
    pub fn new(
        vertex_pointers: VertexPointers<T>,
        shader_program: ShaderProgram,
    ) -> Result<Self> {
        let (vao, vbo) = vertex_pointers.build();
        
        let (vao, quad_vbo) = VertexPointers::new(vao)
            .add(0, 3, GlType::Float, false)
            .add(1, 2, GlType::Float, false)
            .build();

        quad_vbo.load_data(&QUAD);

        let inst = Self {
            vao,
            vbo,
            shader_program,
            _quad_vbo: quad_vbo,
            pixel_size: 1,
        };

        Ok(inst)
    }

    /// Render vertex data.
    /// See the description of [struct::Renderer](Renderer) for an example.
    pub fn render<U: Copy + NumCast>(
        &self,
        texture: &Texture<U>,
        vertex_data: &[T],
        viewport: &Viewport,
        context: &mut Context,
    ) -> Result<()> {
        self.shader_program.enable();
        context.bind_vao(&self.vao);

        unsafe {
            glViewport(
                viewport.position.x,
                viewport.position.y,
                viewport.size.width,
                viewport.size.height,
            );
        }

        // -----------------------------------------------------------------------------
        //     - Set transform / Texture offset -
        //     Set the transform and the texture offset in one.
        //     Load the uv coords from the texture.
        // -----------------------------------------------------------------------------
        texture.bind();

        let num_of_instances = vertex_data.len() as i32;

        self.vbo.load_data(&vertex_data);

        // Clip
        let clip = viewport.projection * viewport.view;
        let clip_uniform_name = CStr::from_bytes_with_nul(b"vp\0").expect("invalid c string");
        self.shader_program
            .set_uniform_matrix(clip, clip_uniform_name)?;

        let pixel_scale_uniform_name = CStr::from_bytes_with_nul(b"pixel_scale\0")
            .expect("invalid c string");

        self.shader_program
            .set_uniform_float(
                self.pixel_size as f32,
                pixel_scale_uniform_name
            )?;

        unsafe {
            glDrawArraysInstanced(
                GL_TRIANGLE_STRIP,
                0,
                QUAD.len() as i32,
                num_of_instances as i32,
            )
        };

        Ok(())
    }
}
