extern crate gl;
extern crate stb_image;

use self::stb_image::stb_image::bindgen::{stbi_load, stbi_image_free};

use self::gl::types::{GLenum, GLint};

use std::os::raw::{c_int, c_void, c_char};
use std::cell::Cell;
use std::ffi::CString;


pub struct Texture2D {
    m_id: u32,
    m_slot: Cell<u32>,
    m_file_path: CString,
}



impl Texture2D {
    pub fn new_from_file(path: CString) -> Texture2D {
        let mut height = 0;
        let mut width = 0;
        let mut channels = 0;
        let mut id = 0;

        unsafe {
            let raw_texture_data = stbi_load(
                path.as_ptr() as *const c_char,
                &mut width,
                &mut height,
                &mut channels,
                0,
            ) as *mut c_void;

            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as c_int,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                raw_texture_data,
            );

            stbi_image_free(raw_texture_data as *mut _); // needed as std::os::raw::c_void
            // is different to libc::c_void
        }

        Texture2D {
            m_id: id,
            m_slot: Cell::<u32>::new(0),
            m_file_path: path,
        }
    }



    pub fn bind_to_slot(&self, slot: GLenum) {
        self.m_slot.set(gl::TEXTURE0 + slot);

        unsafe {
            gl::ActiveTexture(self.m_slot.get());
            gl::BindTexture(gl::TEXTURE_2D, self.m_id);
        }

    }



    pub fn generate_mip_maps(&self) {
        unsafe {
            gl::ActiveTexture(self.m_slot.get());
            gl::BindTexture(gl::TEXTURE_2D, self.m_id);
            gl::GenerateTextureMipmap(gl::TEXTURE_2D);
        }
    }



    pub fn set_texture_property(&self, property: GLenum, value: GLint) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, property, value);
        }
    }
}



impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.m_id);
        }
    }
}



pub struct Texture3D {
    
}