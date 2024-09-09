#![allow(unused_imports, dead_code)]

use std::include_str;

pub mod ascii;
pub mod color;
pub mod debug;
pub mod error;
pub mod matricies;
pub mod model;
pub mod obj;
pub mod render;
pub mod state;
pub mod transform;
pub mod vectors;

use ascii::*;
use matricies::matrix_four_four::*;
use render::render_command::*;
use render::shader::*;
use render::vao::*;
use state::*;
use vectors::*;

pub fn load_resources(state: &mut State, render_api: &impl render::RenderApi) {
    state.basic_shader = Shader::compile(
        include_str!("../engine_resources/shaders/basic.vs"),
        include_str!("../engine_resources/shaders/basic.fs"),
        render_api,
    )
    .unwrap();

    // let sphere_str = include_str!("../engine_resources/sphere.obj");
    // state.model_sphere = obj::load(sphere_str).unwrap();
}

pub fn engine_frame_start(state: &mut State, _input: &Input, _render_api: &impl render::RenderApi) {
    // reset render lists
    state.render_commands = vec![];

    state.frame = state.frame + 1;
}

pub fn engine_frame_end(state: &mut State) {
    state.camera.update_matricies();
}
