use std::ffi::CStr;

use gl33::global_loader::*;
use gl33::*;
use num_traits::cast::NumCast;

use super::shaders::ShaderProgram;
use super::{GlType, Vbo, Vertex, VertexPointers, QUAD, default_vertex_pointers};
use crate::context::{Context, Vao};
use crate::{Result, Viewport, Texture};

// -----------------------------------------------------------------------------
//     - Renderer -
// -----------------------------------------------------------------------------
/// Renderer.
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
    pub pixel_size: u16,
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

        let clip = viewport.projection * viewport.view;

        let clip_uniform_name = CStr::from_bytes_with_nul(b"vp\0").expect("invalid c string");
        self.shader_program
            .set_uniform_matrix(clip, clip_uniform_name)?;

        let pixel_scale_uniform_name = CStr::from_bytes_with_nul(b"pixel_scale\0").expect("invalid c string");
        self.shader_program.set_uniform_float(self.pixel_size as f32, pixel_scale_uniform_name)?;

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
