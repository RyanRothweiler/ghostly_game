use gengar_engine::engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{shader::*, vao::Vao, RenderApi as EngineRenderApiTrait, ShaderType},
    state::State as EngineState,
    vectors::*,
};
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use std::collections::HashMap;

pub static mut GL_CONTEXT: Option<WebGl2RenderingContext> = None;
pub static mut GL_STATE: Option<WebGLState> = None;

pub struct WebGLState {
    pub programs: HashMap<u32, WebGlProgram>,
    pub next_prog_id: u32,
}

pub struct WebGLRenderApi {
    pub gl_clear_color: fn(f32, f32, f32, f32),
    pub gl_clear: fn(),
    pub gl_create_program: fn() -> Option<WebGlProgram>,
    pub gl_create_shader: fn(u32) -> Option<WebGlShader>,
    pub gl_shader_source: fn(&WebGlShader, &str),
    pub gl_compile_shader: fn(&WebGlShader),
    pub gl_get_shader_info_log: fn(&WebGlShader) -> Option<String>,
    pub gl_get_program_info_log: fn(&WebGlProgram) -> Option<String>,
    pub gl_attach_shader: fn(&WebGlProgram, &WebGlShader),
    pub gl_link_program: fn(&WebGlProgram),
    /*
    pub gl_gen_vertex_arrays: fn(i32, *mut u32),
    pub gl_bind_vertex_array: fn(u32),
    pub gl_gen_buffers: fn(i32, *mut u32),
    pub gl_bind_buffer: fn(i32, u32),
    pub gl_buffer_data_v3: fn(i32, &Vec<VecThreeFloat>, i32),
    pub gl_vertex_attrib_pointer_v3: fn(u32),
    pub gl_use_program: fn(u32),
    pub gl_draw_elements: fn(i32, &Vec<u32>),
    pub gl_enable_vertex_attrib_array: fn(u32),
    pub gl_get_uniform_location: fn(u32, &str) -> i32,
    pub gl_uniform_matrix_4fv: fn(i32, i32, bool, &M44),
    */
}

pub fn get_render_api() -> WebGLRenderApi {
    WebGLRenderApi {
        gl_clear_color: gl_clear_color,
        gl_clear: gl_clear,
        gl_create_program: gl_create_program,
        gl_create_shader: gl_create_shader,
        gl_shader_source: gl_shader_source,
        gl_compile_shader: gl_compile_shader,
        gl_get_shader_info_log: gl_get_shader_info_log,
        gl_get_program_info_log: gl_get_program_info_log,
        gl_attach_shader: gl_attach_shader,
        gl_link_program: gl_link_program,
    }
}

impl WebGLRenderApi {
    fn compile_shader(
        &self,
        shader_source: &str,
        shader_type: ShaderType,
    ) -> Result<WebGlShader, EngineError> {
        let gl_shader_type: u32 = match shader_type {
            ShaderType::Vertex => WebGl2RenderingContext::VERTEX_SHADER,
            ShaderType::Fragment => WebGl2RenderingContext::FRAGMENT_SHADER,
        };

        let source: String = "#version 300 es \n ".to_string() + shader_source;

        let shader: WebGlShader = (self.gl_create_shader)(gl_shader_type).unwrap();
        (self.gl_shader_source)(&shader, &source);
        (self.gl_compile_shader)(&shader);

        match (self.gl_get_shader_info_log)(&shader) {
            Some(v) => {
                if v.len() > 0 {
                    return Err(EngineError::ShaderCompilation(v));
                }
            }
            None => {
                return Err(EngineError::ShaderCompilation(
                    "Error getting info".to_string(),
                ));
            }
        }

        Ok(shader)
    }
}

impl EngineRenderApiTrait for WebGLRenderApi {
    fn make_shader_program(
        &self,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<u32, EngineError> {
        let prog: WebGlProgram = (self.gl_create_program)().unwrap();

        let vert_shader = self.compile_shader(vert_shader, ShaderType::Vertex)?;
        let frag_shader = self.compile_shader(frag_shader, ShaderType::Fragment)?;

        (self.gl_attach_shader)(&prog, &vert_shader);
        (self.gl_attach_shader)(&prog, &frag_shader);
        (self.gl_link_program)(&prog);

        match (self.gl_get_program_info_log)(&prog) {
            Some(v) => {
                if v.len() > 0 {
                    return Err(EngineError::ShaderProgramLink(v));
                }
            }
            None => {
                return Err(EngineError::ShaderProgramLink(
                    "Error getting info".to_string(),
                ));
            }
        }

        Ok(0)
    }

    fn create_vao(&self) -> Result<u32, EngineError> {
        /*
        let mut vao_id: u32 = 0;
        (self.gl_gen_vertex_arrays)(1, &mut vao_id);
        Ok(vao_id)
        */

        Ok(0)
    }

    fn vao_upload_v3(
        &self,
        vao: &mut Vao,
        data: &Vec<VecThreeFloat>,
        location: u32,
    ) -> Result<(), EngineError> {
        /*
        (self.gl_bind_vertex_array)(vao.id);

        let mut buf_id: u32 = 0;
        (self.gl_gen_buffers)(1, &mut buf_id);
        vao.add_buffer(buf_id);

        (self.gl_bind_buffer)(GL_ARRAY_BUFFER, buf_id);
        (self.gl_buffer_data_v3)(GL_ARRAY_BUFFER, data, GL_STATIC_DRAW);
        (self.gl_vertex_attrib_pointer_v3)(location);
        (self.gl_enable_vertex_attrib_array)(location);

        (self.gl_bind_vertex_array)(0);
        */

        Ok(())
    }
}

fn gl_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).clear_color(r, g, b, a);
    }
}

pub fn gl_clear() {
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}

pub fn gl_create_program() -> Option<WebGlProgram> {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).create_program();
    }
}

pub fn gl_create_shader(ty: u32) -> Option<WebGlShader> {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).create_shader(ty);
    }
}

pub fn gl_shader_source(shader: &WebGlShader, source: &str) {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).shader_source(shader, source);
    }
}

pub fn gl_compile_shader(shader: &WebGlShader) {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).compile_shader(shader);
    }
}

pub fn gl_get_shader_info_log(shader: &WebGlShader) -> Option<String> {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).get_shader_info_log(shader);
    }
}

pub fn gl_attach_shader(shader: &WebGlProgram, program: &WebGlShader) {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).attach_shader(shader, program);
    }
}

pub fn gl_get_program_info_log(program: &WebGlProgram) -> Option<String> {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).get_program_info_log(program);
    }
}

pub fn gl_link_program(program: &WebGlProgram) {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).link_program(program);
    }
}
