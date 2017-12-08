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
    Geom : Option<GeometryShader>,
    frag : FragmentShader
}



impl ShaderProgram {

    pub fn new(vertex_shader : VertexShader, fragment_shader : FragmentShader) -> ShaderProgram {
        let ID = unsafe{gl::CreateProgram()};
        let vertex_shader = unsafe{gl::AttachShader(self.m_ID, gl::vertex_shader.Get_ID())};
        let fragment_shader = unsafe{gl::AttachShader(self.m_ID, gl::fragment_shader.Get_ID())};
        ShaderProgram{m_ID : self.m_ID, vert : vertex_shader, Geom : None, frag : fragment_shader}
    }



    pub fn new_with_geometry(vertex_shader : VertexShader, geometry_shader : GeometryShader, fragment_shader : FragmentShader) -> ShaderProgram {
        let ID = unsafe{gl::CreateProgram()};
        let vertex_shader = unsafe{gl::AttachShader(self.m_ID, gl::vertex_shader.Get_ID())};
        let geometry_shader = unsafe{gl::AttachShader(self.m_ID, gl::geometry_shader.Get_ID())};
        let fragment_shader = unsafe{gl::AttachShader(self.m_ID, gl::fragment_shader.Get_ID())};
        ShaderProgram{m_ID : ID, vert : vertex_shader, Geom : geometry_shader, frag : fragment_shader}
    }



    pub fn link(&self) -> ShaderLink {
        unsafe{gl::LinkProgram(self.m_ID)};
    }



    pub fn setActive(&self) {
        unsafe{gl::UseProgram(self.m_ID)};
    }



    pub fn setUniformInt(&self, name : CString, value : u32) {
        unimplemented!();
    }



    pub fn setUniformFloat(&self, name : CString, value : f32) {
        unimplemented!();
    }



    pub fn setUniformBool(&self, name : CString, value : bool) {
        unimplemented!();
    }
}



impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unimplemented!();
    }
}
