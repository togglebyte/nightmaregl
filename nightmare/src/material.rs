use std::ffi::CStr;

use nalgebra::Matrix4;

use crate::renderer::shaders::ShaderProgram;
use crate::Result;

pub struct Material<T, U> {
    shader_program: ShaderProgram,
    properties: T,
    uniforms: U,
}

impl<T, U> Material<T, U> {
    fn new(shader_program: ShaderProgram, properties: T, uniforms: U) -> Self {
        Self {
            shader_program,
            properties,
            uniforms,
        }
    }

    fn load_values(&self) {
    }
}

pub struct DefaultUniform {
    pixel_scale: i32,
    clip: i32,
    transform: i32,
}

impl DefaultUniform {
    pub fn new(shader_program: &ShaderProgram) -> Result<Self> {
        let pixel_scale = CStr::from_bytes_with_nul(b"pixel_scale\0").expect("invalid c string");
        let vp = CStr::from_bytes_with_nul(b"vp\0").expect("invalid c string");
        let transform = CStr::from_bytes_with_nul(b"transform\0").expect("invalid c string");

        let inst = Self {
            pixel_scale: shader_program.get_uniform_location(pixel_scale)?,
            clip: shader_program.get_uniform_location(vp)?,
            transform: shader_program.get_uniform_location(transform)?,
        };

        Ok(inst)
    }

    pub fn set_values(&self, shader_program: &ShaderProgram, pixel_size: f32, clip: Matrix4<f32>) {
        shader_program.set_uniform_matrix(clip, self.clip);
        shader_program.set_uniform_float(pixel_size, self.pixel_scale);
    }

    pub fn set_transform(&self, shader_program: &ShaderProgram, transform: &[Matrix4<f32>]) {
        shader_program.set_uniform_matrix_array(transform, self.transform);
    }
}
