extern crate glm;
extern crate gl;

use self::glm::ext::{scale, translate, rotate};
use self::gl::types::GLint;

use std::mem::size_of;

use textures::*;
use shaders::*;
use buffers::*;
use errors::*;

use std::ffi::CString;



pub enum Axis {
    XAxis,
    YAxis,
    ZAxis
}



pub trait Transformable {
    fn translate(&mut self, f32, f32);
    fn rotate(&mut self, f32, Axis);
    fn scale(&mut self, f32, f32);
}



pub trait Draw {
    fn draw(&self) -> DrawStatus;
}



struct PipelineObjects {
    pub prog : ShaderProgram,
    pub tex : Texture2D,
    pub vbo : VertexBufferObject,
    pub ebo : ElementBufferObject
}



fn generate_pipeline_objects(texture_file : CString) -> PipelineObjects {

    let vertex_buffer_object = VertexBufferObject::new();
    let element_buffer_object = ElementBufferObject::new();

    let texture = Texture2D::new(texture_file);

    texture.bind_to_slot(0);
    texture.generate_mip_maps();

    texture.set_texture_property(gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
    texture.set_texture_property(gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
    texture.set_texture_property(gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    texture.set_texture_property(gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

    let vertex_source = r#"#version 450 core 
                        layout (location = 0) in vec3 aPos;
                        layout (location = 1) in vec2 aCoord;

                        out vec2 TexCoord;

                        uniform mat4 trans;

                        void main() { 
                            gl_Position = trans * vec4(aPos, 1.0);
                            TexCoord = aCoord;
                        }"#.to_string();

    let vertex_shader = VertexShader::new_from_string(vertex_source, gl::VERTEX_SHADER);

    match vertex_shader.compile() {
        ShaderCompile::Success => {}
        ShaderCompile::Failed(error) => println!("Failed to Compile Vertex Shader: {}", error)
    }

    let fragement_source = r#"#version 450 core
                            out vec4 FragColor;
                            in vec2 TexCoord;

                            uniform sampler2D ourTexture;

                            void main() {
                                FragColor = texture(ourTexture, TexCoord);
                            }"#.to_string();
    let fragment_shader = FragmentShader::new_from_string(fragement_source, gl::FRAGMENT_SHADER);

    match fragment_shader.compile() {
        ShaderCompile::Success => {}
        ShaderCompile::Failed(error) => println!("Failed to Compile Fragment Shader: {}", error)
    }

    let shader_program = ShaderProgram::new(vertex_shader, fragment_shader);
    let link_status = shader_program.link();
    match link_status {
        ShaderLink::Success => {},
        ShaderLink::Failed(error) => println!("Failed to Link Program: {}", error)
    }

    PipelineObjects{
        prog : shader_program,
        tex : texture,
        vbo : vertex_buffer_object,
        ebo : element_buffer_object
    }
}



fn generate_pipeline_objects_with_geometry(texture_file : CString) -> PipelineObjects {
    
    let vertex_buffer_object = VertexBufferObject::new();
    let element_buffer_object = ElementBufferObject::new();

    let texture = Texture2D::new(texture_file);
    texture.bind_to_slot(0);
    texture.generate_mip_maps();

    texture.set_texture_property(gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
    texture.set_texture_property(gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
    texture.set_texture_property(gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
    texture.set_texture_property(gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

    let vertex_source = r#"#version 450 core
                                                        
                            void main() {
                                gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
                            }"#.to_string();

    let vertex_shader = VertexShader::new_from_string(vertex_source, gl::VERTEX_SHADER);
    match vertex_shader.compile() {
        ShaderCompile::Success => {}
        ShaderCompile::Failed(error) => println!("Failed to Compile Fragment Shader: {}", error)
    }

    let geometry_source = r#"#version 450 core
                            layout(points) in;
                            layout(triangle_strip, max_verticies=GL_MAX_GEOMETRY_OUTPUT_VERTICES) out;
                            
                            uniform int sides;
                            uniform mat4 rotation;
                            uniform mat4 trans;

                            void main() {
                                vec4 pos = vec4(0.0, 1.0, 0.0, 1.0);
                                pos = trans * rotation * pos;
                                gl_Position = pos;
                                EmitVertex();

                                for(int i = 1; i < sides; i++) {
                                    pos = trans * rotation * pos;
                                    gl_Position = pos;
                                    EmitVertex();
                                }
                                EndPrimitives();
                            }"#.to_string();

    let geometry_shader = GeometryShader::new_from_string(geometry_source, gl::GEOMETRY_SHADER);
    match vertex_shader.compile() {
        ShaderCompile::Success => {}
        ShaderCompile::Failed(error) => println!("Failed to Compile Geometry Shader: {}", error)
    }

    let fragment_source = r#"#version 450 core
                            
                            out vec4 FragColor;
                            
                            void main() {
                                FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                            }"#.to_string();

    let fragment_shader = FragmentShader::new_from_string(fragment_source, gl::FRAGMENT_SHADER);
    match fragment_shader.compile() {
        ShaderCompile::Success => {}
        ShaderCompile::Failed(error) => println!("Failed to Compile Fragment Shader: {}", error)
    }

    let shader_program = ShaderProgram::new_with_geometry(vertex_shader, geometry_shader, fragment_shader);
    match shader_program.link() {
        ShaderLink::Success => {},
        ShaderLink::Failed(error) => println!("Failed to Link Program: {}", error)
    }


    PipelineObjects{
        prog : shader_program,
        tex : texture,
        ebo : element_buffer_object,
        vbo : vertex_buffer_object
    }
}



fn scale_and_translate_shape(position_x : f32, position_y : f32, scale_factor : f32) -> glm::Matrix4<f32> {

    let mut transformation_matrix = glm::Matrix4::<f32>::new(glm::Vector4::<f32>::new(1.0, 0.0, 0.0, 0.0),
                                                             glm::Vector4::<f32>::new(0.0, 1.0, 0.0, 0.0),
                                                             glm::Vector4::<f32>::new(0.0, 0.0, 1.0, 0.0),
                                                             glm::Vector4::<f32>::new(0.0, 0.0, 0.0, 1.0),
                                                            );

    transformation_matrix = scale(&transformation_matrix, glm::Vector3::<f32>::new(scale_factor, scale_factor, 0.0));
    transformation_matrix = translate(&transformation_matrix, glm::Vector3::<f32>::new(position_x, position_y, 0.0));

    transformation_matrix
}



pub struct D2Shape {
    pipeline : PipelineObjects,
    num_of_indicies : i32,
    trans : glm::Matrix4<f32>
}



// type aliases for 2D primitives
pub type Triangle = D2Shape;
pub type Square   = D2Shape;



impl D2Shape {
    pub fn new_triangle(position_x : f32, position_y :f32, side_length : f32, texture_file : CString) -> Triangle {
        
        let transformation_matrix = scale_and_translate_shape(position_x, position_y, side_length);

        let verticies  : [f32; 15]    = [-0.5, -0.5, 0.0, 0.0, 1.0,
                                        0.5, -0.5, 0.0, 1.0, 1.0,
                                        0.0,  0.5, 0.0, 0.5,  0.0,];

        let indicies : [u32; 3] = [0, 1, 2];
        
        let pipline_state = generate_pipeline_objects(texture_file);

        pipline_state.vbo.bind_buffer();

        pipline_state.vbo.copy_vertex_array_data(size_of::<[f32; 15]>() as isize, verticies.as_ptr() as *const _, gl::STATIC_DRAW);

        pipline_state.vbo.set_vertex_bindings(0, 3, gl::FLOAT, false, (size_of::<f32>() * 5) as i32, 0 as *const _);
        pipline_state.vbo.set_vertex_bindings(1, 2, gl::FLOAT, false, (size_of::<f32>() * 5) as i32, (3 * size_of::<f32>()) as *const _);

        pipline_state.ebo.bind_buffer();    
        pipline_state.ebo.copy_indicies_data(size_of::<[u32; 3]>() as isize, indicies.as_ptr() as *const _, gl::STATIC_DRAW);

        Triangle{pipeline : pipline_state , num_of_indicies : 3, trans : transformation_matrix}
    }



    pub fn new_polygon(no_of_sides : i32, position_x : f32, position_y : f32, side_lingth : f32, texture_file : CString) -> D2Shape {
        
        let pipeline_state = generate_pipeline_objects_with_geometry(texture_file);

        let transformation_matrix = scale_and_translate_shape(position_x, position_y, side_lingth);

        pipeline_state.ebo.bind_buffer();
        pipeline_state.vbo.bind_buffer();

        let verticies : [f32; 3] = [0.0, 0.0, 0.0];
        pipeline_state.vbo.set_vertex_bindings(0, 3, gl::FLOAT, false, (size_of::<f32>() * 3) as i32, 0 as *const _);

        pipeline_state.vbo.copy_vertex_array_data(size_of::<[f32; 3]>() as isize, verticies.as_ptr() as *const _, gl::STATIC_DRAW);

        D2Shape{pipeline : pipeline_state, num_of_indicies : no_of_sides, trans : transformation_matrix}
    }
}



impl Transformable for D2Shape {
    fn translate(&mut self, trans_x : f32, trans_y : f32) {
        self.trans = translate(&self.trans, glm::Vector3::<f32>::new(trans_x, trans_y, 0.0));
    }



    fn rotate(&mut self, deg : f32, axis_of_rotation : Axis) {
        match axis_of_rotation {
            Axis::XAxis => self.trans = rotate(&self.trans, deg  * (180.0 / glm::ext::consts::pi::<f32, f32>()), glm::Vector3::<f32>::new(1.0, 0.0, 0.0)),
            Axis::YAxis => self.trans = rotate(&self.trans, deg  * (180.0 / glm::ext::consts::pi::<f32, f32>()), glm::Vector3::<f32>::new(0.0, 1.0, 0.0)),
            Axis::ZAxis => self.trans = rotate(&self.trans, deg  * (180.0 / glm::ext::consts::pi::<f32, f32>()), glm::Vector3::<f32>::new(0.0, 0.0, 1.0))
        }
    }



    fn scale(&mut self, scale_factor_x : f32, scale_factor_y : f32) {
        self.trans = scale(&self.trans, glm::Vector3::<f32>::new(scale_factor_x, scale_factor_y, 0.0));
    }
}



impl Draw for D2Shape {

    fn draw(&self) -> DrawStatus{
        
        self.pipeline.vbo.bind_buffer();
        self.pipeline.ebo.bind_buffer();

        self.pipeline.tex.bind_to_slot(0);

        self.pipeline.prog.set_active();
        self.pipeline.prog.set_uniform_mat4(CString::new("trans").unwrap(), self.trans);

        if !self.pipeline.prog.has_geometry_shader() {
            unsafe{ gl::DrawElements(gl::TRIANGLES, self.num_of_indicies, gl::UNSIGNED_INT, 0 as *const _); }
        } else {
            self.pipeline.prog.set_uniform_int(CString::new("sides").unwrap(), self.num_of_indicies as u32);

            let mut transformation_matrix = glm::Matrix4::<f32>::new(glm::Vector4::<f32>::new(1.0, 0.0, 0.0, 0.0),
                                                                     glm::Vector4::<f32>::new(0.0, 1.0, 0.0, 0.0),
                                                                     glm::Vector4::<f32>::new(0.0, 0.0, 1.0, 0.0),
                                                                     glm::Vector4::<f32>::new(0.0, 0.0, 0.0, 1.0),
                                                                    );
            transformation_matrix = rotate(&transformation_matrix
                                                    , (360 / self.num_of_indicies) as f32  * (180.0 / glm::ext::consts::pi::<f32, f32>())
                                                    ,glm::Vector3::<f32>::new(0.0, 0.0, 1.0));

            self.pipeline.prog.set_uniform_mat4(CString::new("rotation").unwrap(), transformation_matrix);

            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 1); }
        }

        let draw_status = unsafe{ gl::GetError()};

        match draw_status {
            gl::NO_ERROR                        => DrawStatus::Success,
            gl::INVALID_VALUE                   => DrawStatus::Failed("invalid value supplied to draw call".to_string()),
            gl::INVALID_OPERATION               => DrawStatus::Failed("invalid draw call for current state".to_string()),
            gl::STACK_OVERFLOW                  => DrawStatus::Failed("stack overflow".to_string()),
            gl::STACK_UNDERFLOW                 => DrawStatus::Failed("stack underflow".to_string()),
            gl::OUT_OF_MEMORY                   => DrawStatus::Failed("out of memory".to_string()),
            gl::INVALID_FRAMEBUFFER_OPERATION   => DrawStatus::Failed("invalid framebuffer operation".to_string()),
            gl::CONTEXT_LOST                    => DrawStatus::Failed("openGL ContextLost".to_string()),
            _                                   => DrawStatus::Failed("UNDEFINED ERROR".to_string()) 
        }
    }
}
