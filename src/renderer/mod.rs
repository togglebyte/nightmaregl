use std::marker::PhantomData;
use std::mem::size_of;

use gl33::global_loader::*;
use gl33::*;
use crate::context::{Context, Vao};
use crate::Vertex;

mod renderer;
mod shaders;

pub use renderer::Renderer;
pub use shaders::{FragmentShader, Shader, ShaderProgram, VertexShader};

/// Vertex buffer object
#[derive(Debug, PartialEq)]
pub struct Vbo<T>(pub(crate) u32, PhantomData<T>);

impl<T> Vbo<T> {
    pub fn new(vbo: u32) -> Self {
        Self(vbo, PhantomData)
    }

    pub(crate) fn enable(&self) {
        unsafe { glBindBuffer(GL_ARRAY_BUFFER, self.0) };
    }

    pub fn load_data(&self, data: &[T]) {
        self.enable();

        let p = data.as_ptr();

        unsafe {
            glBufferData(
                GL_ARRAY_BUFFER,
                (size_of::<T>() * data.len()) as isize,
                p.cast(),
                GL_STATIC_DRAW,
            )
        };
    }
}

impl<T> Drop for Vbo<T> {
    fn drop(&mut self) {
        unsafe { glDeleteBuffers(1, &mut self.0) };
    }
}

// -----------------------------------------------------------------------------
//     - Quad -
//     Vertices making a quad
// -----------------------------------------------------------------------------
const QUAD: [Vertex; 4] = [
    // Top left
    Vertex {
        pos: [0.0, 1.0, 0.0],
        uv_coords: [0.0, 0.0],
    },
    // Bottom left
    Vertex {
        pos: [0.0, 0.0, 0.0],
        uv_coords: [0.0, 1.0],
    },
    // Top right
    Vertex {
        pos: [1.0, 1.0, 0.0],
        uv_coords: [1.0, 0.0],
    },
    // Bottom right
    Vertex {
        pos: [1.0, 0.0, 0.0],
        uv_coords: [1.0, 1.0],
    },
];

// -----------------------------------------------------------------------------
//     - Vertex pointers -
// -----------------------------------------------------------------------------
/// Create a new vertex pointers bound to its own VAO
pub fn new_vertex_pointers<T>(context: &mut Context) -> VertexPointers<T> {
    let vao = context.next_vao();
    context.bind_vao(&vao);
    VertexPointers::<T>::new(vao)
}

pub fn default_vertex_pointers<T>(context: &mut Context) -> VertexPointers<T> {
    let vertex_pointers = new_vertex_pointers(context)
        .with_divisor(1)
        .add(3, 4, GlType::Float, false)
        .add(4, 4, GlType::Float, false)
        .add(5, 4, GlType::Float, false)
        .add(6, 4, GlType::Float, false)
        .add(10, 2, GlType::Float, false)
        .add(11, 2, GlType::Float, false);

    vertex_pointers
}

pub enum GlType {
    Float,
    Int,
}

pub struct VertexPointers<T> {
    next_offset: u32,
    vao: Vao,
    vbo: Vbo<T>,
    divisor: Option<u32>,
}

impl<T> VertexPointers<T> {
    pub(crate) fn new(vao: Vao) -> Self {
        let mut vbo = 0;
        unsafe { glGenBuffers(1, &mut vbo) };
        assert_ne!(vbo, 0);
        let vbo = Vbo::new(vbo);
        unsafe { glBindBuffer(GL_ARRAY_BUFFER, vbo.0) };

        Self {
            next_offset: 0,
            vao,
            vbo,
            divisor: None,
        }
    }

    pub fn with_divisor(mut self, divisor: u32) -> Self {
        self.divisor = Some(divisor);
        self
    }

    pub fn add(
        mut self,
        position: u32,
        param_count: i32,
        gl_type: GlType,
        normalized: bool,
    ) -> Self {
        match gl_type {
            GlType::Float => unsafe {
                glVertexAttribPointer(
                    position,
                    param_count,
                    GL_FLOAT,
                    normalized as u8,
                    size_of::<T>() as i32,
                    self.next_offset as *const _,
                );
            },
            GlType::Int => unsafe {
                glVertexAttribIPointer(
                    position,
                    param_count,
                    GL_FLOAT,
                    size_of::<T>() as i32,
                    self.next_offset as *const _,
                );
            },
        };

        unsafe { glEnableVertexAttribArray(position) };

        match self.divisor {
            Some(divisor) => unsafe { glVertexAttribDivisor(position, divisor) },
            None => {}
        }

        self.next_offset += param_count as u32
            * match gl_type {
                GlType::Float => size_of::<f32>() as u32,
                GlType::Int => size_of::<u32>() as u32,
            };

        self
    }

    pub(crate) fn build(self) -> (Vao, Vbo<T>) {
        let VertexPointers { vbo, vao, .. } = self;
        (vao, vbo)
    }
}
