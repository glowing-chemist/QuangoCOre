extern crate gl;

use Errors::{ShaderCompile, ShaderLink};

use std::fs::File;
use std::io::prelude::*;

use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

trait shader {
    fn compile(&self) -> ShaderCompile;
    fn link() -> ShaderLink;
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

        let SourceLength  = self.m_Source.len() as i32;
        let SourceLengthPointer : *const i32 = &SourceLenght;

        let SourcePToP = &(self.m_Source.as_ptr() as *const i8) as *const *const i8;
        let compileStatus : ShaderCompile = unsafe {
            gl::ShaderSource(self.m_ID, 1, SourcePToP
                            , SourceLenghtPointer);
        
            let mut success : u32 = 0;
            gl::GetShaderiv(self.m_ID, gl::COMPILE_STATUS, *&mut success as *mut c_int);
            if success == 0 {
                ShaderCompile::Success
            } else {
                let mut logLength : u32 = 0;
                gl::GetShaderiv(self.m_ID, gl::INFO_LOG_LENGTH, *&mut logLength as *mut c_int);

                let bufferArray : Vec<u8> = Vec::with_capacity(logLength as usize);

                gl::GetShaderInfoLog(self.m_ID, logLength as c_int, ptr::null_mut(), bufferArray.as_ptr() as *mut c_char);

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
                let SourceLength  = self.m_Source.len() as i32;
        let SourceLengthPointer : *const i32 = &SourceLength;

        let SourcePToP = &(self.m_Source.as_ptr() as *const i8) as *const *const i8;
        let compileStatus : ShaderCompile = unsafe {
            gl::ShaderSource(self.m_ID, 1, SourcePToP
                            , SourceLenghtPointer);
        
            let mut success : u32 = 0;
            gl::GetShaderiv(self.m_ID, gl::COMPILE_STATUS, *&mut success as *mut c_int);
            if success == 0 {
                ShaderCompile::Success
            } else {
                let mut logLength : u32 = 0;
                gl::GetShaderiv(self.m_ID, gl::INFO_LOG_LENGTH, *&mut logLength as *mut c_int);

                let bufferArray : Vec<u8> = Vec::with_capacity(logLength as usize);

                gl::GetShaderInfoLog(self.m_ID, logLength as c_int, ptr::null_mut(), bufferArray.as_ptr() as *mut c_char);

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



pub struct ShaderProgram;

impl ShaderProgram {
    fn new(VertexShader: uint, FragmentShader: uint) -> ShaderProgram {
        let ID = unsafe{gl::CreateProgram()};
        ShaderProgram{m_ID : ID}
    }
    fn Get_ID(&self) -> u32 {
        self.m_ID
    }
}

impl Link for ShaderProgram {
    fn link() {
       unsafe{gl::AttachShader(self.m_ID, gl::VERTEX_SHADER)}; 
       unsafe{gl::AttachShader(self.m_ID, gl::FRAGMENT_SHADER)};
       unsafe{gl::LinkProgram(self.m_ID)};
       let compileStatus : ShaderCompile = unsafe {
           gl::GetProgramiv(self.m_ID, gl::COMPILE_STATUS, *mut success as *mut c_uint);
           if success == 0 {
               ShaderCompile::Success
           } else {
               let mut logLength : u32 = 0;
               gl::GetProgramInfoLog(self.m_ID, logLength as c_uint, ptr::null_mut(), bufferArray.as_ptr() as *mut c_char);
               let bufferArray : Vec<u8> = Vec::with_capacity(logLength as usize);
               ShaderLink::Failed(String::from_utf8(bufferArray).unwrap())
           }
       };
       compileStatus
    }
}

impl Drop for ShaderProgram {
    //delete VertexShader
    //delete FragmentShader
} 
