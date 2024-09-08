use crate::webgl::webgl_render_api::*;

use gengar_engine::engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{shader::*, vao::Vao, RenderApi as EngineRenderApiTrait, ShaderType},
    state::State as EngineState,
    vectors::*,
};
use web_sys::WebGl2RenderingContext;

pub fn render(engine_state: &EngineState, render_api: &WebGLRenderApi, resolution: &VecTwo) {
    (render_api.gl_viewport)(0, 0, resolution.x as i32, resolution.y as i32);

    (render_api.gl_clear_color)(1.0, 0.0, 0.0, 1.0);
    (render_api.gl_clear)();

    /*
    for command in &engine_state.render_commands {
        (render_api.gl_use_program)(command.prog_id);

        for (key, value) in &command.uniforms {
            let loc = (render_api.gl_get_uniform_location)(command.prog_id, key).unwrap();
            match value {
                UniformData::M44(mat) => (render_api.gl_uniform_matrix_4fv)(&loc, false, mat),
                UniformData::Texture(image_id) => todo!(),
            }
        }

        (render_api.gl_bind_vertex_array_engine)(command.vao_id).unwrap();
        (render_api.gl_draw_arrays)(WebGl2RenderingContext::TRIANGLES as i32, &command.indices);

        // todo unbind
    }
    */
}
