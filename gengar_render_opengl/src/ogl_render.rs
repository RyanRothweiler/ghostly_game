#![allow(unused_variables)]

#[allow(non_snake_case)]
pub struct RenderApi {
    pub glClearColor: fn(f32, f32, f32, f32),
    pub glClear: fn(),
}

pub fn render(render_api: &RenderApi) {
    (render_api.glClearColor)(1.0, 0.0, 0.0, 1.0);
    (render_api.glClear)();
}
