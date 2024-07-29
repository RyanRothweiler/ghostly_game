#![allow(unused_variables, dead_code, unused_mut)]

// glext.h
// https://chromium.googlesource.com/external/p3/regal/+/35538fa4176ae0ab6fb8521fc0c7200abbad2e6e/src/apitrace/thirdparty/khronos/GL/glext.h

const GL_VERTEX_SHADER: i32 = 0x8B31;
const GL_FRAGMENT_SHADER: i32 = 0x8B30;
const GL_COMPILE_STATUS: i32 = 0x8B81;
const GL_LINK_STATUS: i32 = 0x8B82;
const GL_ARRAY_BUFFER: i32 = 0x8892;
const GL_STATIC_DRAW: i32 = 0x88E4;

const GL_TRUE: i32 = 1;
const GL_FALSE: i32 = 0;

use gengar_engine::engine::error::Error as EngineError;
use gengar_engine::engine::render::vao::Vao;
use gengar_engine::engine::render::RenderApi as EngineRenderApiTrait;
use gengar_engine::engine::render::ShaderType;
use gengar_engine::engine::vectors::*;

pub struct OglRenderApi {
    pub gl_clear_color: fn(f32, f32, f32, f32),
    pub gl_clear: fn(),
    pub gl_create_shader: fn(i32) -> i32,
    pub gl_shader_source: fn(i32, &str),
    pub gl_compile_shader: fn(i32),
    pub gl_get_shader_iv: fn(i32, i32, *mut i32),
    pub gl_shader_info_log: fn(i32, i32, *mut i32, &mut Vec<u8>),
    pub gl_create_program: fn() -> i32,
    pub gl_attach_shader: fn(i32, i32),
    pub gl_link_program: fn(i32),
    pub gl_gen_vertex_arrays: fn(i32, *mut u32),
    pub gl_bind_vertex_array: fn(u32),
    pub gl_gen_buffers: fn(i32, *mut u32),
    pub gl_bind_buffer: fn(i32, u32),
    pub gl_buffer_data_v3: fn(i32, Vec<VecThreeFloat>, i32),
}

impl OglRenderApi {
    fn shader_info_log(&self, id: i32) -> Result<String, EngineError> {
        let mut string_buf: Vec<u8> = vec![0; 1024];

        let mut output_len: i32 = 0;
        (self.gl_shader_info_log)(id, 1024, &mut output_len, &mut string_buf);

        let error: String = std::ffi::CStr::from_bytes_until_nul(string_buf.as_ref())?
            .to_str()?
            .to_string();

        return Ok(error);
    }

    fn compile_shader(
        &self,
        shader_source: &str,
        shader_type: ShaderType,
    ) -> Result<i32, EngineError> {
        let gl_shader_type: i32 = match shader_type {
            ShaderType::Vertex => GL_VERTEX_SHADER,
            ShaderType::Fragment => GL_FRAGMENT_SHADER,
        };

        let id: i32 = (self.gl_create_shader)(GL_VERTEX_SHADER);

        (self.gl_shader_source)(id, shader_source);
        (self.gl_compile_shader)(id);

        let mut status: i32 = -1;
        (self.gl_get_shader_iv)(id, GL_COMPILE_STATUS, &mut status);
        if status == GL_FALSE {
            let error_info: String = self.shader_info_log(id)?;
            return Err(EngineError::ShaderCompilation(error_info));
        }

        return Ok(id);
    }
}

impl EngineRenderApiTrait for OglRenderApi {
    fn make_shader_program(
        &self,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<i32, EngineError> {
        let vert_id = self.compile_shader(vert_shader, ShaderType::Vertex)?;
        let frag_id = self.compile_shader(frag_shader, ShaderType::Fragment)?;

        let prog_id = (self.gl_create_program)();
        (self.gl_attach_shader)(prog_id, vert_id);
        (self.gl_attach_shader)(prog_id, frag_id);
        (self.gl_link_program)(prog_id);

        let mut status: i32 = -1;
        (self.gl_get_shader_iv)(prog_id, GL_LINK_STATUS, &mut status);
        if status == GL_FALSE {
            let error_info: String = self.shader_info_log(prog_id)?;
            return Err(EngineError::ShaderProgramLink(error_info));
        }

        Ok(0)
    }

    fn create_vao(&self) -> Result<u32, EngineError> {
        let mut vao_id: u32 = 0;
        (self.gl_gen_vertex_arrays)(1, &mut vao_id);
        Ok(vao_id)
    }

    fn vao_upload_v3(&self, vao: &mut Vao, data: Vec<VecThreeFloat>) -> Result<(), EngineError> {
        (self.gl_bind_vertex_array)(vao.id);

        let mut buf_id: u32 = 0;
        (self.gl_gen_buffers)(1, &mut buf_id);
        vao.add_buffer(buf_id);

        (self.gl_bind_buffer)(GL_ARRAY_BUFFER, buf_id);
        (self.gl_buffer_data_v3)(GL_ARRAY_BUFFER, data, GL_STATIC_DRAW);
        // glVertexAttribPointer

        (self.gl_bind_vertex_array)(0);
        Ok(())
    }
}

pub fn render(render_api: &OglRenderApi) {
    (render_api.gl_clear_color)(1.0, 0.0, 0.0, 1.0);
    (render_api.gl_clear)();
}
