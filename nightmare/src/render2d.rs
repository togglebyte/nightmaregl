use std::ffi::CStr;

use nalgebra::Matrix4;
use crate::shaders::{ShaderProgram, Shader};
use crate::vertexpointers::{VertexPointers, ToVertexPointers, Location, ParamCount, Divisor, GlType};
use crate::render::{UniformLocation, instanced_draw};
use crate::{Vao, Vbo, Context, Result, Rect};

pub const VERTEX_SHADER: &[u8] = include_bytes!("shader2d.vert");
pub const FRAGMENT_SHADER: &[u8] = include_bytes!("shader2d.frag");

pub enum Uniform {
    Matrix(Matrix4<f32>),
    Vec4([f32;4]),
    Vec3([f32;3]),
    Float(f32),
}

#[repr(C)]
#[derive(Debug)]
pub struct Model {
    // #[location = 3]
    // #[gl_type = "f32"]
    // #[divisor = 1]
    pub mat: Matrix4<f32>,

    pub texture_rect: Rect,
}

impl Model {
    pub fn new(mat: Matrix4<f32>, texture_rect: Rect) -> Self {
        Self {
            mat,
            texture_rect,
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

        // Texture rect
        vp.add::<Self>(
            Location(7),
            ParamCount(4),
            GlType::Float,
            false,
            Some(Divisor(1))
        );
        
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
    vp_loc: UniformLocation, 
    vbo: Vbo<T>,
    vao: Vao,
}

impl<T: ToVertexPointers> SimpleRenderer<T> {
    pub fn new(context: &mut Context, vp: Matrix4<f32>) -> Result<Self> {
        let vertex_shader = Shader::new_vertex(VERTEX_SHADER)?;
        let fragment_shader = Shader::new_fragment(FRAGMENT_SHADER)?;

        // Setup (and enable) the shader
        let shader_program = ShaderProgram::new(vertex_shader, fragment_shader)?;
        context.enable_shader(&shader_program);

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

        inst.set_view_projection(vp, context);

        Ok(inst)
    }

    pub fn load_data(&mut self, data: &[T], context: &mut Context) {
        self.vbo.load_data(context, data);
    }

    pub fn set_shader(&mut self, shader: ShaderProgram, vp: Matrix4<f32>, context: &mut Context) {
        context.enable_shader(&shader);
        let vp_uniform_name = CStr::from_bytes_with_nul(b"vp\0").unwrap();
        let vp_loc = self.inner.shader_program.get_uniform_location(vp_uniform_name).unwrap();

        self.vp_loc = vp_loc;
        self.inner.shader_program = shader;
        self.set_view_projection(vp, context);
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

    pub fn set_view_projection(&mut self, vp: Matrix4<f32>, context: &mut Context) {
        self.set_uniform(Uniform::Matrix(vp), self.vp_loc, context);
    }

    pub fn get_uniform(&self, name: impl AsRef<str>) -> Option<UniformLocation> {
        let mut bytes = name.as_ref().as_bytes().to_vec();
        bytes.push(b'\0');
        let c_str_name = std::ffi::CStr::from_bytes_with_nul(&bytes).expect("Failed to get uniform name.");
        let loc = self.inner.shader_program.get_uniform_location(c_str_name).ok()?;
        Some(loc)
    }

    pub fn set_uniform(&mut self, uniform: Uniform, loc: UniformLocation, context: &mut Context) {
        context.enable_shader(&self.inner.shader_program);
        match uniform {
            Uniform::Matrix(val) => self.inner.shader_program.set_uniform_matrix(val, loc),
            Uniform::Vec4(val) => self.inner.shader_program.set_uniform_vec4(val, loc),
            Uniform::Vec3(val) => self.inner.shader_program.set_uniform_vec3(val, loc),
            Uniform::Float(val) => self.inner.shader_program.set_uniform_float(val, loc),
        }
    }
}



