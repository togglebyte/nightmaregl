//use std::mem::size_of;
//use gl33::*;
//use gl33::global_loader::*;

///// Attribute location as specified in the layout of a shader.
/////
///// ```text
///// layout (location = 0) in vec3 position;
///// layout (location = 1) in vec2 uv_coords;
///// ```
//pub struct Location(pub(crate) u32);

///// Number of parameters (e.g number of elements in an array)
//pub struct ParamCount(pub(crate) i32);

///// Vertex attribute divisor:
///// By default this is zero. 
/////
///// When using instanced rendering we can set this
///// value to `1`. This means each instance gets the vertex data.
/////
///// ```
///// let divisor = Divisor(1);
///// let data = [
/////     [1, 2, 3], // first instance
/////     [4, 5, 6], // second instance
///// ];
///// ```
/////
///// If it was set to `5` it would mean the first five instances would get the first
///// data etc.
/////
///// ```
///// let divisor = Divisor(1);
///// let data = [
/////     [1, 2, 3], // instance 1 - 5
/////     [4, 5, 6], // 6 - 10
///// ];
///// ```
//pub struct Divisor(pub(crate) u32);

//// -----------------------------------------------------------------------------
////     - Vertex pointers -
//// -----------------------------------------------------------------------------
///// Create a new vertex pointers bound to its own VAO
//pub fn new_vertex_pointers<T>(context: &mut Context) -> VertexPointers<T> {
//    let vao = context.next_vao();
//    context.bind_vao(&vao);
//    VertexPointers::<T>::new(vao)
//}

///// OpenGL data type
//pub enum GlType {
//    /// GL_FLOAT
//    Float,
//    /// GL_INT
//    Int,
//}

///// Vertex pointers.
//pub struct VertexPointers<T> {
//    next_offset: u32,
//    vao: Vao,
//    vbo: Vbo<T>,
//    divisor: Option<Divisor>,
//}

//impl<T> VertexPointers<T> {
//    pub(crate) fn new(vao: Vao) -> Self {
//        let mut vbo = 0;
//        unsafe { glGenBuffers(1, &mut vbo) };
//        assert_ne!(vbo, 0);
//        let vbo = Vbo::new(vbo);
//        unsafe { glBindBuffer(GL_ARRAY_BUFFER, vbo.0) };

//        Self {
//            next_offset: 0,
//            vao,
//            vbo,
//            divisor: None,
//        }
//    }

//    pub fn with_divisor(mut self, divisor: Divisor) -> Self {
//        self.divisor = Some(divisor);
//        self
//    }

//    pub fn add(
//        mut self,
//        location: Location,
//        param_count: ParamCount,
//        gl_type: GlType,
//        normalized: bool,
//    ) -> Self {
//        match gl_type {
//            GlType::Float => unsafe {
//                glVertexAttribPointer(
//                    location.0,
//                    param_count.0,
//                    GL_FLOAT,
//                    normalized as u8,
//                    size_of::<T>() as i32,
//                    self.next_offset as *const _,
//                );
//            },
//            GlType::Int => unsafe {
//                glVertexAttribIPointer(
//                    location.0,
//                    param_count.0,
//                    GL_INT,
//                    size_of::<T>() as i32,
//                    self.next_offset as *const _,
//                );
//            },
//        };

//        unsafe { glEnableVertexAttribArray(location.0) };

//        if let Some(Divisor(divisor)) = self.divisor {
//            unsafe { glVertexAttribDivisor(location.0, divisor) }
//        }

//        let size = match gl_type {
//            GlType::Float => size_of::<f32>() as u32,
//            GlType::Int => size_of::<u32>() as u32,
//        };

//        self.next_offset += param_count.0 as u32 * size;

//        self
//    }

//    pub(crate) fn build(self) -> (Vao, Vbo<T>) {
//        let VertexPointers { vbo, vao, .. } = self;
//        (vao, vbo)
//    }
//}

