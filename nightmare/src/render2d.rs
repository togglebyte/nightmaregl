use std::ffi::CStr;

use nalgebra::Matrix4;
use crate::shaders::{ShaderProgram, Shader};
use crate::vertexpointers::{VertexPointers, ToVertexPointers, Location, ParamCount, Divisor, GlType};
use crate::render::instanced_draw;
use crate::{Vao, Vbo, Context, Result};

#[repr(C)]
pub struct Model {
    // #[location = 3]
    // #[gl_type = "f32"]
    // #[divisor = 1]
    pub mat: Matrix4<f32>,
}

impl Model {
    pub fn new(mat: Matrix4<f32>) -> Self {
        Self {
            mat,
        }
    }
}

impl ToVertexPointers for Model {
    fn vertex_pointer(vp: &mut VertexPointers) {
        for i in 3..7 {
            vp.add::<Self>(
                Location(i),
                ParamCount(4),
                GlType::Float,
                false,
                Some(Divisor(1))
            );
        }
    }
}

// #[derive(VertexData)]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv_coords: [f32; 2],
}

impl ToVertexPointers for Vertex {
    fn vertex_pointer(vp: &mut VertexPointers) {
        vp.add::<Self>(
            Location(0),
            ParamCount(3),
            GlType::Float,
            false,
            None,
        );

        vp.add::<Self>(
            Location(1),
            ParamCount(2),
            GlType::Float,
            false,
            None,
        );
    }
}

pub const QUAD: [Vertex; 4] = [
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

pub const VERTEX_SHADER: &[u8] = include_bytes!("shader.vert");
pub const FRAGMENT_SHADER: &[u8] = include_bytes!("shader.frag");

pub struct Render2d {
    pub shader_program: ShaderProgram,
    quad_vbo: Vbo<Vertex>,
}

impl Render2d {

    pub fn new(context: &mut Context, shader_program: ShaderProgram) -> Self {
        let mut quad_vbo = context.new_vbo();
        quad_vbo.load_data(context, &QUAD);
        Self { shader_program, quad_vbo }
    }

    pub fn new_vao(&self, context: &mut Context) -> Vao {
        let vao = context.new_vao();
        vao.describe::<Vertex>(context, &self.quad_vbo);
        vao
    }

    pub fn render_instanced(&mut self, context: &mut Context, instance_count: usize) {
        context.enable_shader(&self.shader_program);
        instanced_draw(QUAD.len(), instance_count);
    }

}

pub struct SimpleRenderer<T: ToVertexPointers> {
    inner: Render2d,
    vp_loc: i32, 
    vbo: Vbo<T>,
    vao: Vao,
}

impl<T: ToVertexPointers> SimpleRenderer<T> {
    pub fn new(context: &mut Context, vp: Matrix4<f32>) -> Result<Self> {
        let vertex_shader = Shader::new_vertex(VERTEX_SHADER)?;
        let fragment_shader = Shader::new_fragment(FRAGMENT_SHADER)?;

        let shader_program = ShaderProgram::new(vertex_shader, fragment_shader)?;
        let vp_uniform_name = CStr::from_bytes_with_nul(b"vp\0").expect("invalid c string");
        let vp_loc = shader_program.get_uniform_location(vp_uniform_name)?;

        let inner = Render2d::new(context, shader_program);

        let vbo = context.new_vbo();
        let vao = inner.new_vao(context);
        vao.describe::<T>(context, &vbo);

        let mut inst = Self {
            inner,
            vp_loc,
            vbo,
            vao, 
        };

        inst.set_view_projection(vp);

        Ok(inst)
    }

    pub fn load_data(&mut self, data: &[T], context: &mut Context) {
        self.vbo.load_data(context, data);
    }

    pub fn set_shader(&mut self, shader: ShaderProgram, vp: Matrix4<f32>) {
        let vp_uniform_name = CStr::from_bytes_with_nul(b"vp\0").unwrap();
        let vp_loc = self.inner.shader_program.get_uniform_location(vp_uniform_name).unwrap();

        self.vp_loc = vp_loc;
        self.inner.shader_program = shader;
        self.set_view_projection(vp);
    }

    pub fn render_instanced(&mut self, context: &mut Context, instance_count: usize) {
        context.enable_shader(&self.inner.shader_program);
        context.bind_vao(&self.vao);
        context.bind_vbo(&self.vbo);

        self.inner.render_instanced(
            context,
            instance_count,
        );
    }

    pub fn set_view_projection(&mut self, vp: Matrix4<f32>) {
        self.inner.shader_program.enable();
        self.inner.shader_program.set_uniform_matrix(vp, self.vp_loc);
    }
}



