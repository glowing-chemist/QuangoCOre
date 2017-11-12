extern crate gl;

use Errors::{ShaderCompile, ShaderLink};

use std::fs::File;
use std::io::prelude::*;

use std::os::raw::{c_char, c_int};
use std::mem::transmute;

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
        
        let mut vertexFile = File::open(path).expect("file not found");

        let mut contents = String::new();
        vertexFile.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        VertexShader{m_ID : ID, m_Source : contents}
    }
}



impl shader for VertexShader {

    fn compile(&self) -> ShaderCompile {

        let SourceLenght  = self.m_Source.len() as i32;
        let SourceLenghtPointer : *const i32 = &SourceLenght;

        let SourcePointer = self.m_Source.as_ptr();
        let SourcePointerToPointer : *const *const u8 = &SourcePointer;

        unsafe {
            gl::ShaderSource(self.m_ID, 1, SourcePointerToPointer as *const *const c_char
                            , SourceLenghtPointer);
        }
        ShaderCompile::Success
    }
}



pub struct FragmentShader {

}



pub struct ShaderProgram {

}