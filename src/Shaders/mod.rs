extern crate gl;

use Errors::{ShaderCompile, ShaderLink};

use std::fs::File;
use std::io::prelude::*;

use std::os::raw::{c_char, c_int};
use std::ffi::CString;
use std::ptr;

trait shader {
    fn compile(&self) -> ShaderCompile;
}



pub struct VertexShader {
    m_ID : u32,
    m_Source : String,
}



impl VertexShader {

    pub fn new(path : String) -> VertexShader {
        let ID = unsafe {gl::CreateShader(gl::VERTEX_SHADER)};
        
        let mut vertexFile = File::open(path).expect("vertex source file not found");

        let mut contents = String::new();
        vertexFile.read_to_string(&mut contents)
            .expect("failed to read vertex shader Source");

        VertexShader{m_ID : ID, m_Source : contents}

    }



    fn Get_ID(&self) -> u32 {
        self.m_ID
    }
}



impl shader for VertexShader {

    fn compile(&self) -> ShaderCompile {

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
}



impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe{gl::DeleteShader(self.m_ID)}
    }
}



pub struct FragmentShader {
    m_ID : u32,
    m_Source : String
}



impl FragmentShader {

    fn new(path : String) -> FragmentShader {
                let ID = unsafe {gl::CreateShader(gl::VERTEX_SHADER)};
        
        let mut FragmentFile = File::open(path).expect("fragment source file not found");

        let mut contents = String::new();
        FragmentFile.read_to_string(&mut contents)
            .expect("Failed to read Fragment shader source");

        FragmentShader{m_ID : ID, m_Source : contents}
    }


    fn Get_ID(&self) -> u32 {
        self.m_ID
    }
}



impl shader for FragmentShader {

    fn compile(&self) -> ShaderCompile {
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
}



impl Drop for FragmentShader {
    fn drop(&mut self) {
        unsafe{gl::DeleteShader(self.m_ID)}
    }
}



pub struct ShaderProgram {
    m_ID : u32,
    vert : VertexShader,
    frag : FragmentShader
}



impl ShaderProgram {

    pub fn new(vertex_shader : VertexShader, fragment_shader : FragmentShader) -> ShaderProgram {
        unimplemented!();
    }



    pub fn link(&self) -> ShaderLink {
        unimplemented!();
    }



    pub fn setActive(&self) {
        unimplemented!();
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