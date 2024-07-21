#![allow(unused_variables, dead_code, unused_imports)]

/*
// use gengar_engine::engine::*;
pub struct EngineRenderApi {
    pub create_shader: fn() -> i32,
}
*/

#[allow(non_snake_case)]
pub struct RenderApi {
    pub glClearColor: fn(f32, f32, f32, f32),
    pub glClear: fn(),
    pub glCompileShader: fn(u32),
    pub glCreateShader: fn() -> i32,
}

impl RenderApi {
    pub fn make_shader_program(&self) -> i32 {
        let prog_id = (self.glCreateShader)();

        return prog_id;
    }
}

pub fn make_shader_program(render_api: &RenderApi) -> i32 {
    let prog_id = (render_api.glCreateShader)();

    return prog_id;
}

pub fn render(render_api: &RenderApi) {
    (render_api.glClearColor)(1.0, 0.0, 0.0, 1.0);
    (render_api.glClear)();
}
