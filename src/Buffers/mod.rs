extern crate gl;

use std::os::raw::{c_int, c_uint, c_void, c_uchar};


pub struct ElementBufferObject {
    m_ID : u32
}



impl ElementBufferObject {

    pub fn new()  -> ElementBufferObject {
        let mut id : u32 = 0;
        unsafe { gl::GenBuffers(1, &mut id);}
        ElementBufferObject{m_ID : id}
    }



    pub fn bind(&self) {
        unsafe{ gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.m_ID); }
    }



    pub fn copyIndiciesData(bufferSize : isize, buffer : *const c_void, bufferHint : c_uint) { 
        unsafe { gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, bufferSize, buffer, bufferHint); }
    }
}



impl Drop for ElementBufferObject {

    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.m_ID); }
    }
}



pub struct VertexBufferObject {
    m_ID : u32
}



impl VertexBufferObject {

    pub fn setVertexBindings(position : c_uint, size : c_int, bufferHint : c_uint, b : bool, lenght : c_int, offset : *const c_void) {
            unsafe {
                gl::VertexAttribPointer(position, size, bufferHint, b as c_uchar, lenght, offset);
                gl::EnableVertexAttribArray(position);
            }
    }



    pub fn copyVertexArrayData(arraySize : isize, vertexBuffer : *const c_void, bufferHint : c_uint) {
        unsafe { gl::BufferData(gl::ARRAY_BUFFER, arraySize, vertexBuffer, bufferHint); }
    }



    pub fn bindBuffer(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.m_ID); }
    }
}



impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.m_ID); }
    }
}



pub struct VertexArrayObject {
    m_ID : u32
}



impl VertexArrayObject {
    fn new() -> VertexArrayObject {
        let mut id : u32 = 0;
        unsafe {gl::GenVertexArrays(1, &mut id);}
        VertexArrayObject{m_ID : id}
    }



    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.m_ID); }
    }
}



impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.m_ID)}
    }
}