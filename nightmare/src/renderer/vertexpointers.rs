use std::mem::size_of;
use gl33::*;
use gl33::global_loader::*;

use quote::TokenStreamExt;

pub struct Location(pub u32);
pub struct ParamCount(pub i32);
/// Vertex attribute divisor:
/// By default this is zero. 
///
/// When using instanced rendering we can set this
/// value to `1`. This means each instance gets the vertex data.
///
/// ```
/// let divisor = Divisor(1);
/// let data = [
///     [1, 2, 3], // first instance
///     [4, 5, 6], // second instance
/// ];
/// ```
///
/// If it was set to `5` it would mean the first five instances would get the first
/// data etc.
///
/// ```
/// let divisor = Divisor(1);
/// let data = [
///     [1, 2, 3], // instance 1 - 5
///     [4, 5, 6], // 6 - 10
/// ];
/// ```
///
pub struct Divisor(pub u32);

// -----------------------------------------------------------------------------
//     - Vertex pointers -
// -----------------------------------------------------------------------------
/// Create a new vertex pointers bound to its own VAO
pub fn new_vertex_pointers() -> VertexPointers {
    VertexPointers::new()
}

/// OpenGL data type
pub enum GlType {
    /// GL_FLOAT
    Float,
    /// GL_INT
    Int,
}

impl quote::ToTokens for GlType {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let lark: syn::Path = match self {
            GlType::Float => syn::parse_str("nightmare::renderer::vertexpointers::GlType::Float").unwrap(),
            GlType::Int => syn::parse_str("nightmare::renderer::vertexpointers::GlType::Int").unwrap(),
        };

        let tokens = lark.into_token_stream();

        for t in tokens {
            stream.append(t);
        }
    }
}

/// Vertex pointers.
pub struct VertexPointers {
    next_offset: u32,
}

impl VertexPointers {
    pub fn new() -> Self {
        Self {
            next_offset: 0,
        }
    }

    /// Add a vertex attribute, where T is the entire struct:
    /// ```
    /// struct Vertex {
    ///     pos: [f32; 3],
    ///     uv_coords: [f32; 2]
    /// }
    ///
    /// let mut vp = VertexPointers::new();
    /// // Add pos
    /// vp.add::<Vertex>(
    ///     Location(0),
    ///     ParamCount(3),
    ///     GlType::Float,
    ///     false, // normalized
    ///     None,  // divisor
    ///     3,     // field_size
    /// );
    ///
    /// // Add uv_coords
    /// vp.add::<Vertex>(
    ///     Location(1),
    ///     ParamCount(2),
    ///     GlType::Float,
    ///     false, // normalized
    ///     None,  // divisor
    ///     3,     // field_size
    /// );
    /// ```
    pub fn add<T: VertexPointersT>(
        &mut self,
        location: Location,
        param_count: ParamCount,
        gl_type: GlType,
        normalized: bool,
        divisor: Option<Divisor>,
    ) -> &mut Self {

        // let entries = (param_count.0 as u32 + 3) / 4;

        let (size, gl_type) = match gl_type {
            GlType::Float => (size_of::<f32>() as u32, GL_FLOAT),
            GlType::Int => (size_of::<u32>() as u32, GL_INT),
        };

        // for i in 0..entries {
            let location = location.0;// + i;

            unsafe {
                glVertexAttribPointer(
                    location,
                    param_count.0,
                    gl_type,
                    normalized as u8,
                    size_of::<T>() as i32,
                    self.next_offset as *const _,
                );
                glEnableVertexAttribArray(location);

                if let Some(Divisor(divisor)) = divisor {
                    glVertexAttribDivisor(location, divisor);
                }
            };


            self.next_offset += param_count.0 as u32 * size;
        // }

        self
    }
}


pub trait VertexPointersT {
    fn vertex_pointer(vp: &mut VertexPointers);
}
