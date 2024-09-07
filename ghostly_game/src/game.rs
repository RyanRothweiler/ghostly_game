#![allow(unused_imports, unused_variables)]

pub mod state;

use crate::game::state::*;
use gengar_engine::engine::{
    ascii::*,
    matricies::matrix_four_four::*,
    obj,
    render::{
        image::Image, load_image, render_command::RenderCommand, shader::*, vao::*, RenderApi,
    },
    state::Input,
    state::State as EngineState,
    vectors::*,
};
use gengar_render_opengl::ogl_render::*;
use std::{fs::File, path::Path};

// The render_api is hard-coded here instead of using a trait so that we can support hot reloading
#[no_mangle]
pub fn game_init_ogl(game_state: &mut State, render_api: &OglRenderApi) {
    game_init(game_state, render_api);
}

pub fn game_init(game_state: &mut State, render_api: &impl RenderApi) {
    let cube_obj = include_str!("../resources/monkey.obj");
    game_state.cube_model = obj::load(cube_obj).unwrap();

    game_state.cube_vao = Vao::new(render_api);
    game_state
        .cube_vao
        .upload_v3(
            render_api,
            &game_state.cube_model.vertices,
            &game_state.cube_model.indices,
            0,
        )
        .unwrap();
    game_state
        .cube_vao
        .upload_v2(render_api, &game_state.cube_model.uvs, 1)
        .unwrap();

    // load image
    let image = load_image(Path::new(
        "C:/Digital Archive/Game Development/Active/ghostly/ghostly_game/resources/brick.png",
    ))
    .unwrap();

    render_api.upload_texture(&image).unwrap();
}

#[no_mangle]
pub fn game_loop(game_state: &mut State, engine_state: &mut EngineState, input: &Input) {
    // camera controls
    {
        let cam_speed = 0.05;
        if input.keyboard[ASCII_A].pressing {
            engine_state.camera.transform.position.x =
                engine_state.camera.transform.position.x - cam_speed;
        }
        if input.keyboard[ASCII_D].pressing {
            engine_state.camera.transform.position.x =
                engine_state.camera.transform.position.x + cam_speed;
        }
        if input.keyboard[ASCII_S].pressing {
            engine_state.camera.transform.position.y =
                engine_state.camera.transform.position.y - cam_speed;
        }
        if input.keyboard[ASCII_W].pressing {
            engine_state.camera.transform.position.y =
                engine_state.camera.transform.position.y + cam_speed;
        }
        if input.keyboard[ASCII_Q].pressing {
            engine_state.camera.transform.position.z =
                engine_state.camera.transform.position.z + cam_speed;
        }
        if input.keyboard[ASCII_E].pressing {
            engine_state.camera.transform.position.z =
                engine_state.camera.transform.position.z - cam_speed;
        }
        engine_state.camera.update_matricies();
    }

    let offset: f64 = (engine_state.frame as f64) * 0.01;

    let mut mat = M44::new_identity();
    mat.translate(VecThreeFloat::new(0.0, 0.0, 0.0));
    mat.rotate_y(offset);
    mat.rotate_x(offset);
    mat.rotate_z(offset);

    engine_state
        .basic_shader
        .set_uniform("model", UniformData::M44(mat.clone()));

    engine_state.render_commands.push(RenderCommand::new_model(
        &game_state.cube_vao,
        &engine_state.basic_shader,
        game_state.cube_model.indices.clone(),
        &engine_state.camera,
    ));
}
