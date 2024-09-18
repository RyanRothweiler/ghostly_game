use crate::webgl::webgl_render_api::*;

use gengar_engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{shader::*, vao::Vao, RenderApi as EngineRenderApiTrait, ShaderType},
    state::State as EngineState,
    vectors::*,
};
use web_sys::WebGl2RenderingContext;

pub fn render(
    engine_state: &EngineState,
    render_api: &WebGLRenderApi,
    resolution: &VecTwo,
    context: &WebGl2RenderingContext,
) {
    context.viewport(0, 0, resolution.x as i32, resolution.y as i32);

    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context
        .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    for command in &engine_state.render_commands {
        (render_api.gl_use_program)(command.prog_id);

        for (key, value) in &command.uniforms {
            match value {
                UniformData::M44(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => (render_api.gl_uniform_matrix_4fv)(&loc, false, data),

                        // That loc doesn't exist
                        None => {}
                    };
                }
                UniformData::Texture(data) => (render_api.gl_bind_texture)(*data),
                UniformData::VecThree(data) => {
                    todo!();
                }
                UniformData::VecFour(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => (render_api.gl_uniform_4fv)(&loc, data),

                        // That loc doesn't exist
                        None => {}
                    };
                }
            }
        }

        (render_api.gl_bind_vertex_array_engine)(command.vao_id).unwrap();
        (render_api.gl_draw_arrays)(WebGl2RenderingContext::TRIANGLES as i32, &command.indices);
    }
}
