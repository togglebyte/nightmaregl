use std::ffi::CStr;

use log::info;
use gl33::global_loader::*;
use gl33::*;
use nalgebra::Matrix4;

use crate::Result;
use crate::errors::NightmareError;

// -----------------------------------------------------------------------------
//     - Default shaders -
// -----------------------------------------------------------------------------
const DEFAULT_VERTEX: &'static [u8] = include_bytes!("../default.vert");
const DEFAULT_FRAGMENT: &'static [u8] = include_bytes!("../default.frag");
const DEFAULT_FONT: &'static [u8] = include_bytes!("../font.frag");

// -----------------------------------------------------------------------------
//     - Shader types -
// -----------------------------------------------------------------------------
pub struct VertexShader;
pub struct FragmentShader;

// -----------------------------------------------------------------------------
//     - Shader -
// -----------------------------------------------------------------------------
/// Either a vertex shader or a fragment shader.
pub struct Shader<T> {
    pub(crate) id: u32,
    _type: T,
}

impl Shader<VertexShader> {
    /// Create a new vertex shader
    pub fn new_vertex(src: impl AsRef<[u8]>) -> Result<Shader<VertexShader>> {
        let id = glCreateShader(GL_VERTEX_SHADER);
        info!("created new vertex shader: {}", id);
        unsafe { load_shader(id, src.as_ref())? };

        Ok(Self {
            id,
            _type: VertexShader,
        })
    }

    pub fn default_vertex() -> Result<Shader<VertexShader>> {
        Self::new_vertex(&DEFAULT_VERTEX)
    }
}

impl Shader<FragmentShader> {
    /// Create a new fragment shader
    pub fn new_fragment(src: impl AsRef<[u8]>) -> Result<Shader<FragmentShader>> {
        let id = glCreateShader(GL_FRAGMENT_SHADER);
        info!("created new fragment shader: {}", id);
        unsafe { load_shader(id, src.as_ref())? };

        Ok(Self {
            id,
            _type: FragmentShader,
        })
    }

    pub fn default_fragment() -> Result<Shader<FragmentShader>> {
        Self::new_fragment(&DEFAULT_FRAGMENT)
    }

    pub fn default_font() -> Result<Shader<FragmentShader>> {
        Self::new_fragment(&DEFAULT_FONT)
    }
}

// -----------------------------------------------------------------------------
//     - Shader program -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub struct ShaderProgram(pub(crate) u32);

impl ShaderProgram {
    pub(crate) fn attach_shader(&self, shader_id: u32) {
        glAttachShader(self.0, shader_id);
    }

    pub(crate) fn link(&self) -> Result<()> {
        glLinkProgram(self.0);

        let mut shader_compiled = 0;
        unsafe { glGetProgramiv(self.0, GL_LINK_STATUS, &mut shader_compiled) };

        // Failed to compile the shaders
        if shader_compiled == GL_FALSE.0 as i32 {
            let mut error_len = 1024;

            unsafe {
                glGetProgramiv(self.0, GL_INFO_LOG_LENGTH, &mut error_len);

                let mut log: Vec<u8> = Vec::with_capacity(error_len as usize);
                glGetProgramInfoLog(self.0, error_len, &mut error_len, log.as_mut_ptr().cast());

                log.set_len(error_len as usize);

                let error_message = String::from_utf8(log)?;
                return Err(NightmareError::ShaderProgram(error_message));
            }
        }

        Ok(())
    }

    // This should be called after `link()`.
    pub(crate) fn cleanup(&self, shader_id: u32) {
        glDeleteShader(shader_id);
    }

    pub(crate) fn enable(&self) {
        glUseProgram(self.0);
    }

    fn get_uniform_location(&self, name: &CStr) -> Result<i32> {
        let uniform_loc = unsafe { glGetUniformLocation(self.0, name.as_ptr().cast()) };
        if uniform_loc == -1 {
            return Err(NightmareError::ShaderProgram(format!(
                "Invalid uniform name or location: {:?}",
                name
            )));
        }

        Ok(uniform_loc)
    }

    pub(crate) fn set_uniform_matrix(&self, matrix: Matrix4<f32>, name: &CStr) -> Result<()> {
        let uniform_loc = self.get_uniform_location(name)?;
        let transpose = false as u8;
        unsafe { glUniformMatrix4fv(uniform_loc, 1, transpose, matrix.as_ptr()) };

        Ok(())
    }

    // pub(crate) fn set_uniform_vec2(&self, vec: Vector2<f32>, name: &CStr) -> Result<()> {
    //     let uniform_loc = self.get_uniform_location(name)?;
    //     unsafe { glUniform2fv(uniform_loc, 1, vec.as_ptr()) };

    //     Ok(())
    // }

    pub(crate) fn set_uniform_float(&self, f: f32, name: &CStr) -> Result<()> {
        let uniform_loc = self.get_uniform_location(name)?;
        unsafe { glUniform1f(uniform_loc, f) };

        Ok(())
    }

    pub fn default() -> Result<Self> {
        let vertex_shader = Shader::default_vertex()?;
        let fragment_shader = Shader::default_fragment()?;
        Self::new(vertex_shader, fragment_shader)
    }

    pub fn default_font() -> Result<Self> {
        let vertex_shader = Shader::default_vertex()?;
        let fragment_shader = Shader::default_font()?;
        Self::new(vertex_shader, fragment_shader)
    }

    pub fn new(vertex: Shader<VertexShader>, fragment: Shader<FragmentShader>) -> Result<Self> {
        let shader_program = ShaderProgram(glCreateProgram());
        info!("shader program {} created", shader_program.0);

        shader_program.attach_shader(vertex.id);
        shader_program.attach_shader(fragment.id);
        shader_program.link()?;
        shader_program.cleanup(vertex.id);
        shader_program.cleanup(fragment.id);

        Ok(shader_program)
    }
}

// Load a shader.
// The shader will be compiled by the renderer.
unsafe fn load_shader(shader: u32, src: &[u8]) -> Result<()> {
    assert_ne!(shader, 0);

    glShaderSource(shader, 1, &src.as_ptr().cast(), &(src.len() as i32));
    glCompileShader(shader);

    let mut shader_compiled = 0;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &mut shader_compiled);

    // Error compiling the shader
    if shader_compiled == GL_FALSE.0 as i32 {
        let mut error_len = 0;
        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut error_len);

        let mut log: Vec<u8> = Vec::with_capacity(error_len as usize);
        glGetShaderInfoLog(shader, error_len, &mut error_len, log.as_mut_ptr());

        log.set_len(error_len as usize);
        let error_message = String::from_utf8(log)?;

        return Err(NightmareError::Shader(error_message));
    }

    Ok(())
}

