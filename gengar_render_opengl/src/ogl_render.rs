#![allow(unused_variables, dead_code)]

// glext.h
// https://chromium.googlesource.com/external/p3/regal/+/35538fa4176ae0ab6fb8521fc0c7200abbad2e6e/src/apitrace/thirdparty/khronos/GL/glext.h

const GL_VERTEX_SHADER: i32 = 0x8B31;
const GL_FRAGMENT_SHADER: i32 = 0x8B30;

use gengar_engine::engine::render::RenderApi as EnginRenderApiTrait;

pub struct RenderApi {
    pub gl_clear_color: fn(f32, f32, f32, f32),
    pub gl_clear: fn(),
    pub gl_compile_shader: fn(u32),
    pub gl_create_shader: fn(i32) -> i32,
    pub gl_shader_source: fn(i32, &str),
}

impl EnginRenderApiTrait for RenderApi {
    fn make_shader_program(&self, vert_shader: &str, frag_shader: &str) -> i32 {
        let vert_id = (self.gl_create_shader)(GL_VERTEX_SHADER);

        (self.gl_shader_source)(vert_id, "shader stuff here");

        return 0;
    }
}

pub fn render(render_api: &RenderApi) {
    (render_api.gl_clear_color)(1.0, 0.0, 0.0, 1.0);
    (render_api.gl_clear)();
}
