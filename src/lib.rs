mod Shaders;
mod Errors;
mod Buffers;
mod Textures;

use Shaders::{VertexShader, FragmentShader, ShaderProgram};
use Errors::{ShaderCompile, ShaderLink};

#[cfg(test)]
mod tests {
    extern crate gl;
    extern crate glutin;

    use Shaders::{VertexShader, FragmentShader, ShaderProgram};
    use tests::glutin::GlContext;

    #[test]
    fn shader_test() {
        let context = glutin::ContextBuilder::new();
        let window = glutin::WindowBuilder::new();
        let mut events_loop = glutin::EventsLoop::new();
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe { gl_window.make_current().unwrap(); }

        unsafe{ gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _); }

        let vertex_source = "#version 450 core \
                      void main() { \
                            gl_position = vec4(0.0, 0.0, 0.0, 1.0); \
                      }".to_string();

        let vertex_shader : VertexShader = VertexShader::new_from_string(vertex_source, gl::VERTEX_SHADER);
    }
}
