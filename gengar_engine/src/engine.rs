#![allow(unused_variables, unused_imports, non_upper_case_globals)]

use std::include_str;

pub mod color;
pub mod error;
pub mod matricies;
pub mod vectors;

pub mod render;
pub mod state;

use state::*;
use vectors::*;

pub fn load_resources(state: &mut State, render_api: &impl render::RenderApi) {
    let basic_shader_frag = include_str!("../engine_resources/shaders/basic.fs");
    let basic_shader_vert = include_str!("../engine_resources/shaders/basic.vs");

    state.prog_id = render_api
        .make_shader_program(basic_shader_vert, basic_shader_frag)
        .unwrap();

    let mut cube_mesh: Vec<VecThreeFloat> = vec![];
    cube_mesh.push(VecThreeFloat::new(-0.5, -0.5, 0.0));
    cube_mesh.push(VecThreeFloat::new(0.5, -0.5, 0.0));
    cube_mesh.push(VecThreeFloat::new(0.0, 0.5, 0.0));

    let mut cube = render::vao::Vao::new(render_api);
    cube.upload_v3(render_api, cube_mesh, 0);
    state.cube_id = cube.id;
}

pub fn engine_loop(state: &mut State, render_api: &impl render::RenderApi) {
    let indecies: Vec<u32> = vec![0, 1, 2];
    render_api.render(state.prog_id, state.cube_id, &indecies);
}
