use gengar_engine::engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{shader::*, vao::Vao, RenderApi as EngineRenderApiTrait, ShaderType},
    state::State as EngineState,
    vectors::*,
};
use web_sys::{
    console, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation,
    WebGlVertexArrayObject,
};

use js_sys;
use std::collections::HashMap;
use std::mem::size_of;

pub static mut GL_CONTEXT: Option<WebGl2RenderingContext> = None;
pub static mut GL_STATE: Option<WebGLState> = None;

pub struct WebGLState {
    pub programs: HashMap<u32, WebGlProgram>,
    pub next_prog_id: u32,

    pub vaos: HashMap<u32, WebGlVertexArrayObject>,
    pub next_vao_id: u32,
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
    pub gl_create_vertex_array: fn() -> Option<WebGlVertexArrayObject>,
    pub gl_bind_vertex_array: fn(Option<&WebGlVertexArrayObject>),
    pub gl_bind_vertex_array_engine: fn(u32) -> Result<(), EngineError>,
    pub gl_create_buffer: fn() -> Option<WebGlBuffer>,
    pub gl_bind_buffer: fn(u32, Option<&WebGlBuffer>),

    pub gl_buffer_data_v3: fn(u32, &Vec<VecThreeFloat>, u32),
    pub gl_buffer_data_u32: fn(u32, &Vec<u32>, u32),

    pub gl_vertex_attrib_pointer_v3: fn(u32),
    pub gl_enable_vertex_attrib_array: fn(u32),
    pub gl_use_program: fn(u32),
    pub gl_get_uniform_location: fn(u32, &str) -> Option<WebGlUniformLocation>,
    pub gl_uniform_matrix_4fv: fn(&WebGlUniformLocation, bool, &M44),
    pub gl_draw_arrays: fn(i32, &Vec<u32>),
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
        gl_create_vertex_array: gl_create_vertex_array,
        gl_bind_vertex_array: gl_bind_vertex_array,
        gl_bind_vertex_array_engine: gl_bind_vertex_array_engine,
        gl_create_buffer: gl_create_buffer,
        gl_bind_buffer: gl_bind_buffer,
        gl_buffer_data_v3: gl_buffer_data_v3,
        gl_buffer_data_u32: gl_buffer_data_u32,
        gl_vertex_attrib_pointer_v3: gl_vertex_attrib_pointer_v3,
        gl_enable_vertex_attrib_array: gl_enable_vertex_attrib_array,
        gl_use_program: gl_use_program,
        gl_get_uniform_location: gl_get_uniform_location,
        gl_uniform_matrix_4fv: gl_uniform_matrix_4fv,
        gl_draw_arrays: gl_draw_arrays,
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

        let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };
        let prog_id = gl_state.next_prog_id;
        gl_state.next_prog_id = gl_state.next_prog_id + 1;
        gl_state.programs.insert(prog_id, prog);

        Ok(prog_id)
    }

    fn create_vao(&self) -> Result<u32, EngineError> {
        let vao = (self.gl_create_vertex_array)().ok_or(EngineError::CreateVAO)?;

        let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };
        let vao_id = gl_state.next_vao_id;
        gl_state.next_vao_id = gl_state.next_vao_id + 1;
        gl_state.vaos.insert(vao_id, vao);

        Ok(vao_id)
    }

    fn vao_upload_v3(
        &self,
        vao: &mut Vao,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) -> Result<(), EngineError> {
        let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };
        let gl_vao: &WebGlVertexArrayObject = gl_state
            .vaos
            .get(&vao.id)
            .ok_or(EngineError::WebGlMissingVAO)?;

        (self.gl_bind_vertex_array)(Some(gl_vao));

        // setup vertex buffer
        {
            let buf = (self.gl_create_buffer)().ok_or(EngineError::WebGlCreateBuffer)?;

            (self.gl_bind_buffer)(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buf));
            (self.gl_buffer_data_v3)(
                WebGl2RenderingContext::ARRAY_BUFFER,
                data,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            (self.gl_vertex_attrib_pointer_v3)(location);
            (self.gl_enable_vertex_attrib_array)(location);

            (self.gl_bind_buffer)(WebGl2RenderingContext::ARRAY_BUFFER, None);
        }

        // setup the index buffer
        {
            let buf = (self.gl_create_buffer)().ok_or(EngineError::WebGlCreateBuffer)?;
            (self.gl_bind_buffer)(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buf));
            (self.gl_buffer_data_u32)(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                indices,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            // vao.index_buffer =

            // (self.gl_bind_buffer)(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        }

        (self.gl_bind_vertex_array)(None);

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

pub fn gl_create_vertex_array() -> Option<WebGlVertexArrayObject> {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).create_vertex_array();
    }
}

pub fn gl_bind_vertex_array(vao: Option<&WebGlVertexArrayObject>) {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).bind_vertex_array(vao);
    }
}

pub fn gl_bind_vertex_array_engine(vao: u32) -> Result<(), EngineError> {
    unsafe {
        let gl_state: &mut WebGLState = GL_STATE.as_mut().unwrap();

        let gl_vao: &WebGlVertexArrayObject = gl_state
            .vaos
            .get(&vao)
            .ok_or(EngineError::WebGlMissingVAO)?;

        (GL_CONTEXT.as_mut().unwrap()).bind_vertex_array(Some(&gl_vao));
    }

    Ok(())
}

pub fn gl_create_buffer() -> Option<WebGlBuffer> {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).create_buffer();
    }
}

pub fn gl_bind_buffer(target: u32, buf: Option<&WebGlBuffer>) {
    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).bind_buffer(target, buf);
    }
}

fn gl_buffer_data_v3(target: u32, data: &Vec<VecThreeFloat>, usage: u32) {
    unsafe {
        let bytes_total = size_of::<f32>() * 3 * data.len();

        let buf = js_sys::ArrayBuffer::new(bytes_total as u32);
        let buf_view = js_sys::DataView::new(&buf, 0, bytes_total);

        for i in 0..data.len() {
            let byte_offset = size_of::<f32>() * 3 * i;

            buf_view.set_float32_endian(byte_offset, data[i].x as f32, true);
            buf_view.set_float32_endian(byte_offset + size_of::<f32>(), data[i].y as f32, true);
            buf_view.set_float32_endian(
                byte_offset + (size_of::<f32>() * 2),
                data[i].z as f32,
                true,
            );
        }

        (GL_CONTEXT.as_mut().unwrap()).buffer_data_with_opt_array_buffer(target, Some(&buf), usage);
    }
}

fn gl_buffer_data_u32(target: u32, data: &Vec<u32>, usage: u32) {
    unsafe {
        let bytes_total = size_of::<u16>() * data.len();

        let buf = js_sys::ArrayBuffer::new(bytes_total as u32);
        let buf_view = js_sys::DataView::new(&buf, 0, bytes_total);

        for i in 0..data.len() {
            let byte_offset = size_of::<u16>() * i;
            buf_view.set_uint16_endian(byte_offset, u16::try_from(data[i]).unwrap(), true);
        }

        (GL_CONTEXT.as_mut().unwrap()).buffer_data_with_opt_array_buffer(target, Some(&buf), usage);
    }
}

fn gl_vertex_attrib_pointer_v3(location: u32) {
    // stride of 0??
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).vertex_attrib_pointer_with_i32(
            location,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
    }
}

fn gl_enable_vertex_attrib_array(location: u32) {
    // stride of 0??
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).enable_vertex_attrib_array(location);
    }
}

fn gl_use_program(prog: u32) {
    let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };

    let gl_prog: &WebGlProgram = gl_state.programs.get(&prog).unwrap();

    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).use_program(Some(gl_prog));
    }
}

fn gl_get_uniform_location(prog: u32, name: &str) -> Option<WebGlUniformLocation> {
    let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };
    let gl_prog: &WebGlProgram = gl_state.programs.get(&prog).unwrap();

    unsafe {
        return (GL_CONTEXT.as_mut().unwrap()).get_uniform_location(gl_prog, name);
    }
}

pub fn gl_uniform_matrix_4fv(loc: &WebGlUniformLocation, transpose: bool, mat: &M44) {
    unsafe {
        let mut elems: [f32; 16] = [0.0; 16];
        for i in 0..mat.elements.len() {
            elems[i] = mat.elements[i] as f32;
        }
        (GL_CONTEXT.as_mut().unwrap()).uniform_matrix4fv_with_f32_array(
            Some(loc),
            transpose,
            &elems,
        );
    }
}

fn gl_draw_arrays(mode: i32, indices: &Vec<u32>) {
    /*
    let ptr = indecies.as_ptr() as *const libc::c_void;
    unsafe {
        (extern_global_glDrawElements.unwrap())(
            mode,
            i32::try_from(indecies.len()).unwrap(),
            GL_UNSIGNED_INT,
            0 as *const libc::c_void,
        )
    }
    */

    // unsafe { (GL_CONTEXT.as_mut().unwrap()).draw_arrays(mode as u32, 0, indices.len() as i32) }

    /*
    let ptr = indecies.as_ptr() as *const libc::c_void;
    unsafe {
        (extern_global_glDrawElements.unwrap())(
            mode,
            i32::try_from(indecies.len()).unwrap(),
            GL_UNSIGNED_INT,
            0 as *const libc::c_void,
        )
    }
    */
    // setup the index buffer
    {
        /*
        let buf = gl_create_buffer()
            .ok_or(EngineError::WebGlCreateBuffer)
            .unwrap();
        gl_bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buf));
        gl_buffer_data_u32(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            indices,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        */

        // vao.index_buffer =

        // (self.gl_bind_buffer)(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
    }

    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).draw_elements_with_i32(
            mode as u32,
            indices.len() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        )
    }
}
