use std::marker::PhantomData;
use std::mem::size_of;

use crate::context::{Context, Vao};
use crate::Vertex;
use gl33::global_loader::*;
use gl33::*;

pub mod default;
pub(crate) mod shaders;
pub mod vertexpointers;

pub use vertexpointers::{VertexPointers, GlType, new_vertex_pointers};
pub use shaders::{FragmentShader, Shader, ShaderProgram, VertexShader};

/// Delete this rubbish
pub fn instanced_draw(instance_count: i32) {
    unsafe {
        glDrawArraysInstanced(
            GL_TRIANGLE_STRIP,
            0,
            instance_count,
            1,
        )
    };
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
