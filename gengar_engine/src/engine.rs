#![allow(unused_imports)]

use std::include_str;

pub mod color;
pub mod error;
pub mod matricies;
pub mod vectors;

pub mod render;
pub mod state;

use matricies::matrix_four_four::*;
use render::render_command::*;
use render::shader::*;
use render::vao::*;
use state::*;
use vectors::*;

pub fn load_resources(state: &mut State, render_api: &impl render::RenderApi) {
    let basic_shader_frag = include_str!("../engine_resources/shaders/basic.fs");
    let basic_shader_vert = include_str!("../engine_resources/shaders/basic.vs");

    state.basic_shader = Shader::compile(basic_shader_vert, basic_shader_frag, render_api).unwrap();
}

pub fn engine_frame_start(state: &mut State, render_api: &impl render::RenderApi) {
    // reset render lists
    state.render_commands = vec![];

    state.frame = state.frame + 1;

    let offset: f64 = (state.frame as f64) * 0.001;

    let mut mat = MatrixFourFour::new_identity();
    mat.translate(VecThreeFloat::new(0.0, offset, 0.0));

    let first = VecThreeFloat::new(-0.5, -0.5, 0.0);
    let second = VecThreeFloat::new(0.5, -0.5, 0.0);
    let third = VecThreeFloat::new(0.0, 0.5, 0.0);

    let first = MatrixFourFour::apply_vec_three(&mat, &first);
    let second = MatrixFourFour::apply_vec_three(&mat, &second);
    let third = MatrixFourFour::apply_vec_three(&mat, &third);

    state.cube = render::vao::Vao::new(render_api);
    state
        .cube
        .upload_v3(render_api, vec![first, second, third], 0);

    state.render_commands.push(RenderCommand::new_model(
        &state.cube,
        &state.basic_shader,
        vec![0, 1, 2],
    ));
}
