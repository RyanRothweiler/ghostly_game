use crate::webgl::webgl_render_api::*;

use gengar_engine::{
    error::Error as EngineError,
    matricies::matrix_four_four::*,
    render::{
        camera::*, render_command::*, shader::*, vao::Vao, RenderApi as EngineRenderApiTrait,
        ShaderType,
    },
    state::State as EngineState,
    vectors::*,
};
use web_sys::WebGl2RenderingContext;

pub fn render(
    es: &mut EngineState,
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

    render_list(&mut es.render_commands, &es.camera, render_api, context);

    // Debug render lists
    render_list(
        gengar_engine::debug::get_render_list(),
        &es.camera,
        render_api,
        context,
    );
    render_list(
        &mut es.game_debug_render_commands,
        &es.camera,
        render_api,
        context,
    );
}

fn render_list(
    render_commands: &mut Vec<RenderCommand>,
    camera: &Camera,
    render_api: &WebGLRenderApi,
    context: &WebGl2RenderingContext,
) {
    for command in render_commands {
        (render_api.gl_use_program)(command.prog_id);

        // setup the camera transforms
        command
            .uniforms
            .insert("view".to_string(), UniformData::M44(camera.view_mat));
        command.uniforms.insert(
            "projection".to_string(),
            UniformData::M44(camera.projection_mat),
        );
        command.uniforms.insert(
            "viewPos".to_string(),
            UniformData::VecThree(camera.transform.local_position),
        );

        for (key, value) in &command.uniforms {
            match value {
                UniformData::M44(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => (render_api.gl_uniform_matrix_4fv)(&loc, false, data),

                        // That loc doesn't exist
                        None => {}
                    };
                }
                UniformData::Texture(data) => {
                    todo!("fix image slot data")
                    // (render_api.gl_bind_texture)(*data)
                }
                UniformData::VecThree(data) => {
                    match (render_api.gl_get_uniform_location)(command.prog_id, key) {
                        Some(loc) => (render_api.gl_uniform_3fv)(&loc, data),

                        // That loc doesn't exist
                        None => {}
                    };
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
