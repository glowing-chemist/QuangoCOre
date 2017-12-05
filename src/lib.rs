pub mod Shaders;
pub mod Errors;
pub mod Buffers;
pub mod Textures;

use Shaders::{VertexShader, FragmentShader, ShaderProgram};
use Errors::{ShaderCompile, ShaderLink};

#[cfg(test)]
mod tests {
    extern crate gl;
    extern crate glutin;

    use Shaders::{VertexShader, FragmentShader, ShaderProgram};
    use Errors::{ShaderCompile, ShaderLink};
    use tests::glutin::GlContext;

    #[test]
    fn shader_test() {
        let context = glutin::ContextBuilder::new();
        let window = glutin::WindowBuilder::new();
        let events_loop = glutin::EventsLoop::new();
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe { gl_window.make_current().unwrap(); }

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        let vertex_source = r#"#version 450 core
                                void main() {
                                    gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
                                }"#.to_string();

        let vertex_shader : VertexShader = VertexShader::new_from_string(vertex_source, gl::VERTEX_SHADER);

        match vertex_shader.compile() {
            ShaderCompile::Success => {}
            ShaderCompile::Failed(error) => panic!(error)
        }
    }
}
