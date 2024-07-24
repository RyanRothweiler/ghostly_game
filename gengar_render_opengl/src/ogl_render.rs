#![allow(unused_variables, dead_code, unused_mut)]

// glext.h
// https://chromium.googlesource.com/external/p3/regal/+/35538fa4176ae0ab6fb8521fc0c7200abbad2e6e/src/apitrace/thirdparty/khronos/GL/glext.h

const GL_VERTEX_SHADER: i32 = 0x8B31;
const GL_FRAGMENT_SHADER: i32 = 0x8B30;
const GL_COMPILE_STATUS: i32 = 0x8B81;

use gengar_engine::engine::render::RenderApi as EnginRenderApiTrait;
use gengar_engine::engine::render::ShaderType;

pub struct RenderApi {
    pub gl_clear_color: fn(f32, f32, f32, f32),
    pub gl_clear: fn(),
    pub gl_create_shader: fn(i32) -> i32,
    pub gl_shader_source: fn(i32, &str),
    pub gl_compile_shader: fn(i32),
    pub gl_get_shader_iv: fn(i32, i32, *mut i32),
    pub gl_shader_info_log: fn(i32, i32, *mut i32, &mut Vec<u8>),
}

impl RenderApi {
    fn compile_shader(&self, shader_source: &str, shader_type: ShaderType) -> i32 {
        let gl_shader_type: i32 = match shader_type {
            ShaderType::Vertex => GL_VERTEX_SHADER,
            ShaderType::Fragment => GL_FRAGMENT_SHADER,
        };

        let id: i32 = (self.gl_create_shader)(GL_VERTEX_SHADER);

        println!("shader {shader_source}");

        (self.gl_shader_source)(id, shader_source);
        (self.gl_compile_shader)(id);

        let mut status: i32 = 0;
        (self.gl_get_shader_iv)(id, GL_COMPILE_STATUS, &mut status);
        if status == 0 {
            eprintln!("Error compiling vertex shader");

            let mut string_buf: Vec<u8> = vec![0; 1024];

            let mut output_len: i32 = 0;
            (self.gl_shader_info_log)(id, 1024, &mut output_len, &mut string_buf);

            let read_string: String = std::ffi::CStr::from_bytes_until_nul(string_buf.as_ref())
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            println!("error {read_string}");
        }

        return id;
    }
}

impl EnginRenderApiTrait for RenderApi {
    fn make_shader_program(&self, vert_shader: &str, frag_shader: &str) -> i32 {
        self.compile_shader(vert_shader, ShaderType::Vertex);

        return 0;
    }
}

pub fn render(render_api: &RenderApi) {
    (render_api.gl_clear_color)(1.0, 0.0, 0.0, 1.0);
    (render_api.gl_clear)();
}
