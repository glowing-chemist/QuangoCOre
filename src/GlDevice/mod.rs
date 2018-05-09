extern crate glm;

use buffers::*;
use camera::*;
use errors::*;
use shaders::*;
use textures::*;

pub struct DrawableObject {
    mIndexBufferOffset : usize,
    mIndexSize : usize,
    mTextureBufferOffset : usize,

    mPosition : glm::Matrix4<f32>,
}

pub struct GlDevice {
    mVertexShader : VertexShader,
    mFragmentShader : FragmentShader,
    mPipeline : ShaderProgram,

    mVAO : VertexArrayObject,
    mEBO : ElementBufferObject,
    mVBO : VertexBufferObject,

    mSceneObjects : Vec<DrawableObject>,
    mObjectTextures : Vec<Texture2D>,

    mcellShadingLookupTexture : Texture3D,

    mCamera : Camera,
}