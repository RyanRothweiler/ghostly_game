use gengar_engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{image::Image, shader::*, vao::Vao, RenderApi as EngineRenderApiTrait, ShaderType},
    state::State as EngineState,
    vectors::*,
};
use web_sys::{
    console, WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
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

    pub textures: HashMap<u32, WebGlTexture>,
    pub next_texture_id: u32,
}

pub struct WebGLRenderApi {
    pub gl_bind_vertex_array_engine: fn(u32) -> Result<(), EngineError>,
    pub gl_use_program: fn(u32),
    pub gl_get_uniform_location: fn(u32, &str) -> Option<WebGlUniformLocation>,
    pub gl_draw_arrays: fn(i32, &Vec<u32>),
    pub gl_viewport: fn(i32, i32, i32, i32),
    pub gl_bind_texture: fn(u32),

    pub gl_uniform_matrix_4fv: fn(&WebGlUniformLocation, bool, &M44),
    pub gl_uniform_4fv: fn(&WebGlUniformLocation, &VecFour),
    pub gl_uniform_3fv: fn(&WebGlUniformLocation, &VecThreeFloat),
}

pub fn get_render_api() -> WebGLRenderApi {
    WebGLRenderApi {
        gl_bind_vertex_array_engine: gl_bind_vertex_array_engine,
        gl_use_program: gl_use_program,
        gl_get_uniform_location: gl_get_uniform_location,
        gl_draw_arrays: gl_draw_arrays,
        gl_viewport: gl_viewport,
        gl_bind_texture: gl_bind_texture,

        gl_uniform_matrix_4fv: gl_uniform_matrix_4fv,
        gl_uniform_4fv: gl_uniform_4fv,
        gl_uniform_3fv: gl_uniform_3fv,
    }
}

impl WebGLRenderApi {
    fn compile_shader(
        &self,
        shader_source: &str,
        shader_type: ShaderType,
        context: &WebGl2RenderingContext,
    ) -> Result<WebGlShader, EngineError> {
        let gl_shader_type: u32 = match shader_type {
            ShaderType::Vertex => WebGl2RenderingContext::VERTEX_SHADER,
            ShaderType::Fragment => WebGl2RenderingContext::FRAGMENT_SHADER,
        };

        let source: String = "#version 300 es \n ".to_string() + shader_source;

        let shader: WebGlShader = context.create_shader(gl_shader_type).unwrap();
        context.shader_source(&shader, &source);
        context.compile_shader(&shader);

        match context.get_shader_info_log(&shader) {
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
        let context = unsafe { GL_CONTEXT.as_mut().ok_or(EngineError::WebGlNoContext)? };

        let prog: WebGlProgram = context.create_program().unwrap();

        let vert_shader = self.compile_shader(vert_shader, ShaderType::Vertex, &context)?;
        let frag_shader = self.compile_shader(frag_shader, ShaderType::Fragment, &context)?;

        context.attach_shader(&prog, &vert_shader);
        context.attach_shader(&prog, &frag_shader);
        context.link_program(&prog);

        match context.get_program_info_log(&prog) {
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
        let context = unsafe { GL_CONTEXT.as_mut().ok_or(EngineError::WebGlNoContext)? };

        let vao = context
            .create_vertex_array()
            .ok_or(EngineError::CreateVAO)?;

        let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };
        let vao_id = gl_state.next_vao_id;
        gl_state.next_vao_id = gl_state.next_vao_id + 1;
        gl_state.vaos.insert(vao_id, vao);

        Ok(vao_id)
    }

    fn vao_upload_v3(
        &self,
        vao: &Vao,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) -> Result<(), EngineError> {
        let gl_state: &mut WebGLState =
            unsafe { GL_STATE.as_mut().ok_or(EngineError::WebGlNoState)? };
        let context = unsafe { GL_CONTEXT.as_mut().ok_or(EngineError::WebGlNoContext)? };

        let gl_vao: &WebGlVertexArrayObject = gl_state
            .vaos
            .get(&vao.id)
            .ok_or(EngineError::WebGlMissingVAO)?;

        context.bind_vertex_array(Some(gl_vao));

        // setup vertex buffer
        {
            let buf = context
                .create_buffer()
                .ok_or(EngineError::WebGlCreateBuffer)?;

            context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buf));
            gl_buffer_data_v3(
                WebGl2RenderingContext::ARRAY_BUFFER,
                data,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            vertex_attrib_pointer_v3(location);
            context.enable_vertex_attrib_array(location);

            context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        }

        // setup the index buffer
        {
            let buf = context
                .create_buffer()
                .ok_or(EngineError::WebGlCreateBuffer)?;
            context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buf));
            gl_buffer_data_u32(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                indices,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        context.bind_vertex_array(None);

        Ok(())
    }

    fn vao_upload_v2(
        &self,
        vao: &Vao,
        data: &Vec<VecTwo>,
        location: u32,
    ) -> Result<(), EngineError> {
        let context = unsafe { GL_CONTEXT.as_mut().ok_or(EngineError::WebGlNoContext)? };

        let gl_state: &mut WebGLState =
            unsafe { GL_STATE.as_mut().ok_or(EngineError::WebGlNoState)? };
        let gl_vao: &WebGlVertexArrayObject = gl_state
            .vaos
            .get(&vao.id)
            .ok_or(EngineError::WebGlMissingVAO)?;

        context.bind_vertex_array(Some(gl_vao));

        let buf = context
            .create_buffer()
            .ok_or(EngineError::WebGlCreateBuffer)?;

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buf));
        gl_buffer_data_v2(
            WebGl2RenderingContext::ARRAY_BUFFER,
            data,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        gl_vertex_attrib_pointer_v2(location);
        context.enable_vertex_attrib_array(location);

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        context.bind_vertex_array(None);

        Ok(())
    }

    fn upload_texture(&self, data: &Image) -> Result<u32, EngineError> {
        let context = unsafe { GL_CONTEXT.as_mut().unwrap() };

        let tex: WebGlTexture = context
            .create_texture()
            .ok_or(EngineError::WebGlCreateTexture)?;

        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));

        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );

        let mip_level: i32 = 0;
        let border: i32 = 0;

        let gl_internal_format: i32 = WebGl2RenderingContext::SRGB8 as i32;
        let image_format: u32 = WebGl2RenderingContext::RGB as u32;
        let image_pixel_format: u32 = WebGl2RenderingContext::UNSIGNED_BYTE as u32;

        context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                mip_level,
                gl_internal_format,
                data.width as i32,
                data.height as i32,
                border,
                image_format,
                image_pixel_format,
                &data.data,
                0,
            )
            .unwrap();

        let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };
        let tex_id = gl_state.next_texture_id;
        gl_state.next_texture_id = gl_state.next_texture_id + 1;
        gl_state.textures.insert(tex_id, tex);

        Ok(tex_id)
    }
}

fn gl_bind_vertex_array_engine(vao: u32) -> Result<(), EngineError> {
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

fn gl_buffer_data_v2(target: u32, data: &Vec<VecTwo>, usage: u32) {
    unsafe {
        let bytes_total = size_of::<f32>() * 2 * data.len();

        let buf = js_sys::ArrayBuffer::new(bytes_total as u32);
        let buf_view = js_sys::DataView::new(&buf, 0, bytes_total);

        for i in 0..data.len() {
            let byte_offset = size_of::<f32>() * 2 * i;
            buf_view.set_float32_endian(byte_offset, data[i].y as f32, true);
            buf_view.set_float32_endian(byte_offset + size_of::<f32>(), data[i].y as f32, true);
        }

        (GL_CONTEXT.as_mut().unwrap()).buffer_data_with_opt_array_buffer(target, Some(&buf), usage);
    }
}

fn vertex_attrib_pointer_v3(location: u32) {
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

fn gl_vertex_attrib_pointer_v2(location: u32) {
    // stride of 0??
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).vertex_attrib_pointer_with_i32(
            location,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
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

fn gl_uniform_matrix_4fv(loc: &WebGlUniformLocation, transpose: bool, mat: &M44) {
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

fn gl_uniform_4fv(loc: &WebGlUniformLocation, data: &VecFour) {
    unsafe {
        let elems: [f32; 4] = [data.x as f32, data.y as f32, data.z as f32, data.w as f32];
        (GL_CONTEXT.as_mut().unwrap()).uniform4fv_with_f32_array(Some(loc), &elems);
    }
}

fn gl_uniform_3fv(loc: &WebGlUniformLocation, data: &VecThreeFloat) {
    unsafe {
        let elems: [f32; 3] = [data.x as f32, data.y as f32, data.z as f32];
        (GL_CONTEXT.as_mut().unwrap()).uniform3fv_with_f32_array(Some(loc), &elems);
    }
}

fn gl_draw_arrays(mode: i32, indices: &Vec<u32>) {
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).draw_elements_with_i32(
            mode as u32,
            indices.len() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        )
    }
}

fn gl_viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).viewport(x, y, width, height);
    }
}

fn gl_bind_texture(id: u32) {
    let gl_state: &mut WebGLState = unsafe { GL_STATE.as_mut().unwrap() };
    let gl_texture: &WebGlTexture = gl_state.textures.get(&id).unwrap();

    unsafe {
        (GL_CONTEXT.as_mut().unwrap())
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&gl_texture));
    }
}
