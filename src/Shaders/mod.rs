extern crate gl;

use Errors::{ShaderCompile, ShaderLink};

use std::fs::File;
use std::io::prelude::*;

use std::os::raw::{c_char, c_int};
use std::ffi::CString;
use std::ptr;

use self::gl::types::GLenum;



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

        let SourceLenght  = self.m_Source.len() as i32;
        let SourceLenghtPointer : *const i32 = &SourceLenght;

        let SourcePToP = &(self.m_Source.as_ptr() as *const i8) as *const *const i8;
        let compileStatus : ShaderCompile = unsafe {
            gl::ShaderSource(self.m_ID, 1, SourcePToP
                            , SourceLenghtPointer);
        
            let mut success : u32 = 0;
            gl::GetShaderiv(self.m_ID, gl::COMPILE_STATUS, *&mut success as *mut c_int);
            if success == 0 {
                ShaderCompile::Success
            } else {
                let mut logLenght : u32 = 0;
                gl::GetShaderiv(self.m_ID, gl::INFO_LOG_LENGTH, *&mut logLenght as *mut c_int);

                let bufferArray : Vec<u8> = Vec::with_capacity(logLenght as usize);

                gl::GetShaderInfoLog(self.m_ID, logLenght as c_int, ptr::null_mut(), bufferArray.as_ptr() as *mut c_char);

                ShaderCompile::Failed(String::from_utf8(bufferArray).unwrap())
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
        let vertex_shader = unsafe{gl::AttachShader(self.m_ID, gl::vertex_shader.Get_ID)};
        let fragment_shader = unsafe{gl::AttachShader(self.m_ID, gl::fragment_shader.Get_ID)};
        ShaderProgram{m_ID : self.m_ID, vert : vertex_shader, Geom : None, frag : fragment_shader}
    }



    pub fn new_with_geometry(vertex_shader : VertexShader, geometry_shader : GeometryShader, fragment_shader : FragmentShader) -> ShaderProgram {
        let ID = unsafe{gl::CreateProgram()};
        let vertex_shader = unsafe{gl::AttachShader(self.m_ID, gl::vertex_shader.Get_ID)};
        let geometry_shader = unsafe{gl::AttachShader(self.m_ID, gl::geometry_shader.Get_ID)};
        let fragment_shader = unsafe{gl::AttachShader(self.m_ID, gl::fragment_shader.Get_ID)};
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