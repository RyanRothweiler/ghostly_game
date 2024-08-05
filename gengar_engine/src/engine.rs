#![allow(unused_imports)]

use std::include_str;

pub mod color;
pub mod error;
pub mod matricies;
pub mod vectors;

pub mod render;
pub mod state;

use matricies::matrix_four_four::*;
use state::*;
use vectors::*;

pub fn load_resources(state: &mut State, render_api: &impl render::RenderApi) {
    let basic_shader_frag = include_str!("../engine_resources/shaders/basic.fs");
    let basic_shader_vert = include_str!("../engine_resources/shaders/basic.vs");

    state.prog_id = render_api
        .make_shader_program(basic_shader_vert, basic_shader_frag)
        .unwrap();
}

pub fn engine_loop(state: &mut State, render_api: &impl render::RenderApi) {
    state.frame = state.frame + 1;

    let offset: f64 = (state.frame as f64) * 0.001;

    let mut mat = MatrixFourFour::new_identity();
    mat.translate(VecThreeFloat::new(offset, offset, 0.0));

    let first = VecThreeFloat::new(-0.5, -0.5, 0.0);
    let second = VecThreeFloat::new(0.5, -0.5, 0.0);
    let third = VecThreeFloat::new(0.0, 0.5, 0.0);

    let first = MatrixFourFour::apply_vec_three(&mat, &first);
    let second = MatrixFourFour::apply_vec_three(&mat, &second);
    let third = MatrixFourFour::apply_vec_three(&mat, &third);

    let mut cube = render::vao::Vao::new(render_api);
    cube.upload_v3(render_api, vec![first, second, third], 0);
    state.cube_id = cube.id;

    let indecies: Vec<u32> = vec![0, 1, 2];
    render_api.render(state.prog_id, state.cube_id, &indecies);
}
