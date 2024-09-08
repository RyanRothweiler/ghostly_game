#![allow(unused_variables, unused_imports, dead_code)]

use gengar_engine::engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{image::Image, shader::*, vao::Vao, RenderApi as EngineRenderApiTrait, ShaderType},
    state::State as EngineState,
    vectors::*,
};

const GL_VERTEX_SHADER: i32 = 0x8B31;
const GL_FRAGMENT_SHADER: i32 = 0x8B30;
const GL_COMPILE_STATUS: i32 = 0x8B81;
const GL_LINK_STATUS: i32 = 0x8B82;
const GL_ARRAY_BUFFER: i32 = 0x8892;
const GL_STATIC_DRAW: i32 = 0x88E4;
const GL_ELEMENT_ARRAY_BUFFER: i32 = 0x8893;
const GL_TEXTURE_2D: i32 = 0x0DE1;

const GL_TRIANGLES: i32 = 0x0004;

// const GL_TRUE: i32 = 1;
const GL_FALSE: i32 = 0;

const RG: i32 = 0x8227;
const RG16: i32 = 0x822C;
const RG16F: i32 = 0x822F;
const RG16I: i32 = 0x8239;
const RG16UI: i32 = 0x823A;
const RG16_SNORM: i32 = 0x8F99;
const RG32F: i32 = 0x8230;
const RG32I: i32 = 0x823B;
const RG32UI: i32 = 0x823C;
const RG8: i32 = 0x822B;
const RG8I: i32 = 0x8237;
const RG8UI: i32 = 0x8238;
const RG8_SNORM: i32 = 0x8F95;
const RGB: i32 = 0x1907;
const RGB10: i32 = 0x8052;
const RGB10_A2: i32 = 0x8059;
const RGB10_A2UI: i32 = 0x906F;
const RGB12: i32 = 0x8053;
const RGB16: i32 = 0x8054;
const RGB16F: i32 = 0x881B;
const RGB16I: i32 = 0x8D89;
const RGB16UI: i32 = 0x8D77;
const RGB16_SNORM: i32 = 0x8F9A;
const RGB32F: i32 = 0x8815;
const RGB32I: i32 = 0x8D83;
const RGB32UI: i32 = 0x8D71;
const RGB4: i32 = 0x804F;
const RGB5: i32 = 0x8050;
const RGB5_A1: i32 = 0x8057;
const RGB8: i32 = 0x8051;
const RGB8I: i32 = 0x8D8F;
const RGB8UI: i32 = 0x8D7D;
const RGB8_SNORM: i32 = 0x8F96;
const RGB9_E5: i32 = 0x8C3D;
const RGBA: i32 = 0x1908;
const RGBA12: i32 = 0x805A;
const RGBA16: i32 = 0x805B;
const RGBA16F: i32 = 0x881A;
const RGBA16I: i32 = 0x8D88;
const RGBA16UI: i32 = 0x8D76;
const RGBA16_SNORM: i32 = 0x8F9B;
const RGBA2: i32 = 0x8055;
const RGBA32F: i32 = 0x8814;
const RGBA32I: i32 = 0x8D82;
const RGBA32UI: i32 = 0x8D70;
const RGBA4: i32 = 0x8056;
const RGBA8: i32 = 0x8058;
const RGBA8I: i32 = 0x8D8E;
const RGBA8UI: i32 = 0x8D7C;
const RGBA8_SNORM: i32 = 0x8F97;
const RGBA_INTEGER: i32 = 0x8D99;
const RGB_INTEGER: i32 = 0x8D98;
const RG_INTEGER: i32 = 0x8228;

const UNSIGNED_BYTE: i32 = 0x1401;
const UNSIGNED_BYTE_2_3_3_REV: i32 = 0x8362;
const UNSIGNED_BYTE_3_3_2: i32 = 0x8032;
const UNSIGNED_INT: i32 = 0x1405;
const UNSIGNED_INT_10F_11F_11F_REV: i32 = 0x8C3B;
const UNSIGNED_INT_10_10_10_2: i32 = 0x8036;
const UNSIGNED_INT_24_8: i32 = 0x84FA;
const UNSIGNED_INT_2_10_10_10_REV: i32 = 0x8368;
const UNSIGNED_INT_5_9_9_9_REV: i32 = 0x8C3E;
const UNSIGNED_INT_8_8_8_8: i32 = 0x8035;
const UNSIGNED_INT_8_8_8_8_REV: i32 = 0x8367;
const UNSIGNED_INT_SAMPLER_1D: i32 = 0x8DD1;
const UNSIGNED_INT_SAMPLER_1D_ARRAY: i32 = 0x8DD6;
const UNSIGNED_INT_SAMPLER_2D: i32 = 0x8DD2;
const UNSIGNED_INT_SAMPLER_2D_ARRAY: i32 = 0x8DD7;
const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: i32 = 0x910A;
const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: i32 = 0x910D;
const UNSIGNED_INT_SAMPLER_2D_RECT: i32 = 0x8DD5;
const UNSIGNED_INT_SAMPLER_3D: i32 = 0x8DD3;
const UNSIGNED_INT_SAMPLER_BUFFER: i32 = 0x8DD8;
const UNSIGNED_INT_SAMPLER_CUBE: i32 = 0x8DD4;
const UNSIGNED_INT_VEC2: i32 = 0x8DC6;
const UNSIGNED_INT_VEC3: i32 = 0x8DC7;
const UNSIGNED_INT_VEC4: i32 = 0x8DC8;
const UNSIGNED_NORMALIZED: i32 = 0x8C17;
const UNSIGNED_SHORT: i32 = 0x1403;
const UNSIGNED_SHORT_1_5_5_5_REV: i32 = 0x8366;
const UNSIGNED_SHORT_4_4_4_4: i32 = 0x8033;
const UNSIGNED_SHORT_4_4_4_4_REV: i32 = 0x8365;
const UNSIGNED_SHORT_5_5_5_1: i32 = 0x8034;
const UNSIGNED_SHORT_5_6_5: i32 = 0x8363;
const UNSIGNED_SHORT_5_6_5_REV: i32 = 0x8364;

const GL_DEPTH_TEST: u32 = 0x0B71;
const GL_LEQUAL: u32 = 0x0203;
const GL_GEQUAL: u32 = 0x0206;

const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
const GL_LINEAR: u32 = 0x2601;

// Platform must provide these methods
pub struct OglRenderApi {
    pub gl_enable: fn(u32),
    pub gl_depth_func: fn(u32),

    pub gl_clear_color: fn(f32, f32, f32, f32),
    pub gl_clear: fn(),
    pub gl_create_shader: fn(i32) -> u32,
    pub gl_shader_source: fn(u32, &str),
    pub gl_compile_shader: fn(u32),
    pub gl_get_shader_iv: fn(u32, i32, *mut i32),
    pub gl_shader_info_log: fn(u32, i32, *mut i32, &mut Vec<u8>),
    pub gl_create_program: fn() -> u32,
    pub gl_attach_shader: fn(u32, u32),
    pub gl_link_program: fn(u32),
    pub gl_gen_vertex_arrays: fn(i32, *mut u32),
    pub gl_bind_vertex_array: fn(u32),
    pub gl_gen_buffers: fn(i32, *mut u32),
    pub gl_bind_buffer: fn(i32, u32),

    pub gl_buffer_data_v3: fn(i32, &Vec<VecThreeFloat>, i32),
    pub gl_buffer_data_v2: fn(i32, &Vec<VecTwo>, i32),
    pub gl_buffer_data_u32: fn(i32, &Vec<u32>, i32),

    pub gl_vertex_attrib_pointer_v3: fn(u32),
    pub gl_vertex_attrib_pointer_v2: fn(u32),

    pub gl_use_program: fn(u32),
    pub gl_draw_elements: fn(i32, &Vec<u32>),
    pub gl_enable_vertex_attrib_array: fn(u32),
    pub gl_get_uniform_location: fn(u32, &str) -> i32,
    pub gl_uniform_matrix_4fv: fn(i32, i32, bool, &M44),
    pub gl_gen_textures: fn(i32, *mut u32),
    pub gl_bind_texture: fn(i32, u32),
    pub gl_tex_image_2d: fn(u32, i32, u32, u32, &Image),
    pub gl_tex_parameter_i: fn(u32, u32, i32),
}

impl OglRenderApi {
    fn shader_info_log(&self, id: u32) -> Result<String, EngineError> {
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
    ) -> Result<u32, EngineError> {
        let gl_shader_type: i32 = match shader_type {
            ShaderType::Vertex => GL_VERTEX_SHADER,
            ShaderType::Fragment => GL_FRAGMENT_SHADER,
        };

        let source: String = "#version 330 core \n ".to_string() + shader_source;

        let id: u32 = (self.gl_create_shader)(gl_shader_type);

        (self.gl_shader_source)(id, &source);
        (self.gl_compile_shader)(id);

        let mut status: i32 = -1;
        (self.gl_get_shader_iv)(id, GL_COMPILE_STATUS, &mut status);
        if status == GL_FALSE {
            let error_info: String = self.shader_info_log(id)?;
            return Err(EngineError::ShaderCompilation(error_info));
        }

        Ok(id)
    }
}

impl EngineRenderApiTrait for OglRenderApi {
    fn make_shader_program(
        &self,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<u32, EngineError> {
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
    }

    fn create_vao(&self) -> Result<u32, EngineError> {
        let mut vao_id: u32 = 0;
        (self.gl_gen_vertex_arrays)(1, &mut vao_id);
        Ok(vao_id)
    }

    fn vao_upload_v3(
        &self,
        vao: &mut Vao,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) -> Result<(), EngineError> {
        (self.gl_bind_vertex_array)(vao.id);

        // setup the vertex buffer
        {
            let mut buf_id: u32 = 0;
            (self.gl_gen_buffers)(1, &mut buf_id);

            (self.gl_bind_buffer)(GL_ARRAY_BUFFER, buf_id);
            (self.gl_buffer_data_v3)(GL_ARRAY_BUFFER, data, GL_STATIC_DRAW);
            (self.gl_vertex_attrib_pointer_v3)(location);
            (self.gl_enable_vertex_attrib_array)(location);

            (self.gl_bind_buffer)(GL_ARRAY_BUFFER, 0);
        }

        (self.gl_bind_vertex_array)(0);

        Ok(())
    }

    fn vao_upload_v2(
        &self,
        vao: &mut Vao,
        data: &Vec<VecTwo>,
        location: u32,
    ) -> Result<(), EngineError> {
        (self.gl_bind_vertex_array)(vao.id);

        let mut buf_id: u32 = 0;
        (self.gl_gen_buffers)(1, &mut buf_id);

        (self.gl_bind_buffer)(GL_ARRAY_BUFFER, buf_id);
        (self.gl_buffer_data_v2)(GL_ARRAY_BUFFER, data, GL_STATIC_DRAW);
        (self.gl_vertex_attrib_pointer_v2)(location);
        (self.gl_enable_vertex_attrib_array)(location);

        (self.gl_bind_buffer)(GL_ARRAY_BUFFER, 0);

        (self.gl_bind_vertex_array)(0);

        Ok(())
    }

    fn upload_texture(&self, image: &Image) -> Result<u32, EngineError> {
        let mut tex_id: u32 = 0;
        (self.gl_gen_textures)(1, &mut tex_id);
        (self.gl_bind_texture)(GL_TEXTURE_2D, tex_id);

        (self.gl_tex_parameter_i)(
            GL_TEXTURE_2D as u32,
            GL_TEXTURE_MAG_FILTER,
            GL_LINEAR as i32,
        );
        (self.gl_tex_parameter_i)(
            GL_TEXTURE_2D as u32,
            GL_TEXTURE_MIN_FILTER,
            GL_LINEAR as i32,
        );

        (self.gl_tex_image_2d)(
            GL_TEXTURE_2D as u32,
            RGB,
            RGB as u32,
            UNSIGNED_BYTE as u32,
            &image,
        );

        Ok(tex_id)
    }
}

pub fn render(engine_state: &EngineState, render_api: &OglRenderApi) {
    (render_api.gl_enable)(GL_DEPTH_TEST);
    (render_api.gl_depth_func)(GL_LEQUAL);

    (render_api.gl_clear_color)(1.0, 0.0, 0.0, 1.0);
    (render_api.gl_clear)();

    for command in &engine_state.render_commands {
        (render_api.gl_use_program)(command.prog_id);

        for (key, value) in &command.uniforms {
            let loc = (render_api.gl_get_uniform_location)(command.prog_id, key);
            match value {
                UniformData::M44(mat) => (render_api.gl_uniform_matrix_4fv)(loc, 1, false, mat),
                UniformData::Texture(image_id) => {
                    (render_api.gl_bind_texture)(GL_TEXTURE_2D, *image_id)
                }
            }
        }

        (render_api.gl_bind_vertex_array)(command.vao_id);
        (render_api.gl_draw_elements)(GL_TRIANGLES, &command.indices);
    }
}
