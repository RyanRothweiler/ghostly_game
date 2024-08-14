#![allow(unused_imports, unused_variables)]

pub mod state;

use std::path::Path;

use gengar_engine::engine::render::render_command::RenderCommand;
use gengar_engine::engine::state::State as EngineState;
use gengar_engine::engine::*;

use crate::game::state::*;

pub fn game_init(game_state: &mut State) {
    // obj::load(Path::new("testinghere")).unwrap();

    let cube_obj = include_str!("../resources/cube.obj");
    game_state.cube_model = obj::load(cube_obj).unwrap();
}

pub fn game_loop(game_state: &mut State, engine_state: &mut EngineState) {
    /*
    engine_state.render_commands.push(RenderCommand::new_model(
        &game_state.cube_model.vertices,
        &state.basic_shader,
        vec![0, 1, 2],
        &state.camera,
    ));
    */
}
