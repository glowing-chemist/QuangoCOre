extern crate gl;
extern crate stb_image;

use self::stb_image::stb_image::bindgen::{stbi_load, stbi_image_free};

use self::gl::types::{GLenum, GLint};

use std::os::raw::{c_int, c_void, c_char};
use std::ffi::CString;


pub struct Texture2D {
    m_ID : u32,
    m_slot : u32,
    m_filePath : CString,
}



impl Texture2D {

    pub fn new(path : CString) -> Texture2D {
        let mut height = 0;
        let mut width = 0;
        let mut channels = 0;
        let mut id = 0;

        unsafe {
            let rawTextureData = stbi_load(path.as_ptr() as *const c_char, &mut width, &mut height, &mut channels, 0) as *mut c_void;

            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as c_int, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, rawTextureData);

            stbi_image_free(rawTextureData as *mut _); // needed as std::os::raw::c_void is different to libc::c_void
        }

        Texture2D{m_ID : id, m_slot : 0, m_filePath : path}
    }



    pub fn bindToSlot(&mut self, slot : GLenum) {
        self.m_slot = gl::TEXTURE0 + slot;

        unsafe {
            gl::ActiveTexture(self.m_slot);
            gl::BindTexture(gl::TEXTURE_2D, self.m_ID);
        }

    }



    pub fn generateMipMaps(&self) {
        unsafe {
            gl::ActiveTexture(self.m_slot);
            gl::BindTexture(gl::TEXTURE_2D, self.m_ID);
            gl::GenerateTextureMipmap(gl::TEXTURE_2D);
        }
    }



    pub fn setTextureProperty(&self, property : GLenum, value : GLint) {
        unsafe{ gl::TexParameteri(gl::TEXTURE_2D, property, value); }
    }
}



impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe{ gl::DeleteTextures(1, &self.m_ID); }
    }
}