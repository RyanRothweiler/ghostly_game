#![allow(unused_imports, unused_variables)]

pub mod state;

use std::path::Path;

use gengar_engine::engine::{
    matricies::matrix_four_four::*, obj, render::render_command::RenderCommand, render::shader::*,
    render::vao::*, state::State as EngineState, vectors::*,
};

use crate::game::state::*;

pub fn game_init(
    game_state: &mut State,
    render_api: &impl gengar_engine::engine::render::RenderApi,
) {
    let cube_obj = include_str!("../resources/monkey.obj");
    game_state.cube_model = obj::load(cube_obj).unwrap();

    game_state.cube_vao = Vao::new(render_api);
    game_state
        .cube_vao
        .upload_v3(render_api, &game_state.cube_model.vertices, 0);
}

pub fn game_loop(game_state: &mut State, engine_state: &mut EngineState) {
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
