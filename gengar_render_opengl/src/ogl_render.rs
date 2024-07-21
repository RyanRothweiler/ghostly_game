#![allow(unused_variables, dead_code)]

use gengar_engine::engine::render::RenderApi as EnginRenderApiTrait;

#[allow(non_snake_case)]
pub struct RenderApi {
    pub glClearColor: fn(f32, f32, f32, f32),
    pub glClear: fn(),
    pub glCompileShader: fn(u32),
    pub glCreateShader: fn() -> i32,
}

impl EnginRenderApiTrait for RenderApi {
    fn make_shader_program(&self) -> i32 {
        let prog_id = (self.glCreateShader)();

        return prog_id;
    }
}

pub fn render(render_api: &RenderApi) {
    (render_api.glClearColor)(1.0, 0.0, 0.0, 1.0);
    (render_api.glClear)();
}
