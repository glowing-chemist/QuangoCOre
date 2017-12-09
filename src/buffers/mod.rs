extern crate gl;

use std::os::raw::{c_int, c_uint, c_void, c_uchar};


pub struct ElementBufferObject {
    m_id: u32,
}



impl ElementBufferObject {
    pub fn new() -> ElementBufferObject {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        ElementBufferObject { m_id: id }
    }



    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.m_id);
        }
    }



    pub fn copy_indicies_data(buffer_size: isize, buffer: *const c_void, buffer_hint: c_uint) {
        unsafe {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, buffer_size, buffer, buffer_hint);
        }
    }
}



impl Drop for ElementBufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.m_id);
        }
    }
}



pub struct VertexBufferObject {
    m_id: u32,
}



impl VertexBufferObject {
    pub fn set_vertex_bindings(
        position: c_uint,
        size: c_int,
        buffer_hint: c_uint,
        b: bool,
        lenght: c_int,
        offset: *const c_void,
    ) {
        unsafe {
            gl::VertexAttribPointer(position, size, buffer_hint, b as c_uchar, lenght, offset);
            gl::EnableVertexAttribArray(position);
        }
    }



    pub fn copy_vertex_array_aata(
        array_size: isize,
        vertex_buffer: *const c_void,
        buffer_hint: c_uint,
    ) {
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER, array_size, vertex_buffer, buffer_hint);
        }
    }



    pub fn bind_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.m_id);
        }
    }
}



impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.m_id);
        }
    }
}



pub struct VertexArrayObject {
    m_id: u32,
}



impl VertexArrayObject {
    fn new() -> VertexArrayObject {
        let mut id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArrayObject { m_id: id }
    }



    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.m_id);
        }
    }
}



impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.m_id) }
    }
}
