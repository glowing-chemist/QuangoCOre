extern crate gl;

use Errors::{ShaderCompile, ShaderLink};

use std::fs::File;
use std::io::prelude::*;

use std::os::raw::{c_char, c_int};
use std::ffi::CString;
use std::ptr;

use self::gl::types::{GLenum, GLint, GLchar};



pub struct Shader {
    m_ID : u32,
    m_Source : String,
}



impl Shader {

    pub fn new_from_file(path : String, shader_type : GLenum) -> Shader {
        let id = unsafe {gl::CreateShader(shader_type)};
        
        let mut vertex_file = File::open(path).expect("shader source file not found");

        let mut contents = String::new();
        vertex_file.read_to_string(&mut contents)
            .expect("failed to read shader Source");

        Shader{m_ID : id, m_Source : contents}

    }



    pub fn new_from_string(shader_source : String, shader_type : GLenum) -> Shader {
        let id = unsafe{ gl::CreateShader(shader_type) };

        Shader{m_ID : id, m_Source : shader_source}
    }



    pub fn compile(&self) -> ShaderCompile {

        let shader_source = CString::new(self.m_Source.as_bytes()).unwrap();

        let compileStatus = unsafe {
            gl::ShaderSource(self.m_ID, 1, &shader_source.as_ptr()
                            , ptr::null());
        
            gl::CompileShader(self.m_ID);

            let mut success : GLint = 0;
            gl::GetShaderiv(self.m_ID, gl::COMPILE_STATUS, &mut success);
            if success == gl::TRUE as GLint {
                ShaderCompile::Success
            } else {
                let mut logLenght : GLint = 0;
                gl::GetShaderiv(self.m_ID, gl::INFO_LOG_LENGTH, &mut logLenght);

                let mut log_buffer = Vec::with_capacity(logLenght as usize);

                gl::GetShaderInfoLog(self.m_ID, logLenght, ptr::null_mut(), log_buffer.as_mut_ptr() as *mut GLchar);

                ShaderCompile::Failed(String::from_utf8(log_buffer).unwrap())
            }
        };

        compileStatus
    }



    pub fn Get_ID(&self) -> u32 {
        self.m_ID
    }
}



impl Drop for Shader {
    fn drop(&mut self) {
        unsafe{gl::DeleteShader(self.m_ID)}
    }
}



pub type VertexShader = Shader;
pub type GeometryShader = Shader;
pub type FragmentShader = Shader;



pub struct ShaderProgram {
    m_ID : u32,
    vert : VertexShader,
    geom : Option<GeometryShader>,
    frag : FragmentShader
}



impl ShaderProgram {

    pub fn new(vertex_shader : VertexShader, fragment_shader : FragmentShader) -> ShaderProgram {
        let id = unsafe{ gl::CreateProgram() };
        unsafe{
            gl::AttachShader(id, vertex_shader.Get_ID());
            gl::AttachShader(id, fragment_shader.Get_ID());
        }

        ShaderProgram{m_ID : id, vert : vertex_shader, geom : None, frag : fragment_shader}
    }



    pub fn new_with_geometry(vertex_shader : VertexShader, geometry_shader : GeometryShader, fragment_shader : FragmentShader) -> ShaderProgram {
        let id = unsafe{ gl::CreateProgram() };
        unsafe{
            gl::AttachShader(id, vertex_shader.Get_ID());
            gl::AttachShader(id, geometry_shader.Get_ID());
            gl::AttachShader(id, fragment_shader.Get_ID());
        }

        ShaderProgram{m_ID : id, vert : vertex_shader, geom : Some(geometry_shader), frag : fragment_shader}    }



    pub fn link(&self) -> ShaderLink {
        unsafe{ gl::LinkProgram(self.m_ID); }

                let link_status = unsafe {

            let mut success : GLint = 0;
            gl::GetShaderiv(self.m_ID, gl::LINK_STATUS, &mut success);
            if success == gl::TRUE as GLint {
                ShaderLink::Success
            } else {
                let mut log_length : GLint = 0;
                gl::GetShaderiv(self.m_ID, gl::INFO_LOG_LENGTH, &mut log_length);

                let mut log_buffer = Vec::with_capacity(log_length as usize);

                gl::GetShaderInfoLog(self.m_ID, log_length, ptr::null_mut(), log_buffer.as_mut_ptr() as *mut GLchar);

                ShaderLink::Failed(String::from_utf8(log_buffer).unwrap())
            }
        };

        link_status
    }



    pub fn set_active(&self) {
        unsafe{ gl::UseProgram(self.m_ID); }
    }



    pub fn set_uniform_int(&self, name : CString, value : u32) {
        unsafe{ gl::ProgramUniform1i(self.m_ID, gl::GetUniformLocation(self.m_ID, name.as_ptr()), value as i32); }
    }



    pub fn set_uniform_float(&self, name : CString, value : f32) {
        unsafe{ gl::ProgramUniform1f(self.m_ID, gl::GetUniformLocation(self.m_ID, name.as_ptr()), value); };
    }



    pub fn set_uniform_bool(&self, name : CString, value : bool) {
        unsafe{ gl::ProgramUniform1i(self.m_ID, gl::GetUniformLocation(self.m_ID, name.as_ptr()), value as i32); };
    }
}



impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe{ gl::DeleteProgram(self.m_ID); }
    }
}