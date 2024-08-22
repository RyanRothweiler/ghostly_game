use gengar_engine::engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{shader::*, vao::Vao, RenderApi as EngineRenderApiTrait, ShaderType},
    state::State as EngineState,
    vectors::*,
};
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub static mut GL_CONTEXT: Option<WebGl2RenderingContext> = None;

pub struct WebGLRenderApi {
    pub gl_clear_color: fn(f32, f32, f32, f32),
    pub gl_clear: fn(),
}

pub fn get_render_api() -> WebGLRenderApi {
    WebGLRenderApi {
        gl_clear_color: gl_clear_color,
        gl_clear: gl_clear,
    }
}

impl EngineRenderApiTrait for WebGLRenderApi {
    fn make_shader_program(
        &self,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<u32, EngineError> {
        /*
        let vert_id = self.compile_shader(vert_shader, ShaderType::Vertex)?;
        let frag_id = self.compile_shader(frag_shader, ShaderType::Fragment)?;

        let prog_id: u32 = (self.gl_create_program)();
        (self.gl_attach_shader)(prog_id, vert_id);
        (self.gl_attach_shader)(prog_id, frag_id);
        (self.gl_link_program)(prog_id);

        let mut status: i32 = -1;
        (self.gl_get_shader_iv)(prog_id, GL_LINK_STATUS, &mut status);
        if status == GL_FALSE {
            let error_info: String = self.shader_info_log(prog_id)?;
            return Err(EngineError::ShaderProgramLink(error_info));
        }

        // delete the shaders?

        Ok(prog_id)
        */
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
