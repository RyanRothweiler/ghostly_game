#![allow(unused_imports)]

use std::include_str;

pub mod ascii;
pub mod color;
pub mod error;
pub mod matricies;
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
}

pub fn engine_frame_start(state: &mut State, input: &Input, render_api: &impl render::RenderApi) {
    // reset render lists
    state.render_commands = vec![];

    state.frame = state.frame + 1;

    let _offset: f64 = (state.frame as f64) * 0.001;

    let mut mat = M44::new_identity();
    mat.translate(VecThreeFloat::new(0.0, 0.0, 0.0));

    state
        .basic_shader
        .set_uniform("model", UniformData::M44(mat.clone()));

    let first = VecThreeFloat::new(-0.5, -0.5, 0.0);
    let second = VecThreeFloat::new(0.5, -0.5, 0.0);
    let third = VecThreeFloat::new(0.0, 0.5, 0.0);

    let first = M44::apply_vec_three(&mat, &first);
    let second = M44::apply_vec_three(&mat, &second);
    let third = M44::apply_vec_three(&mat, &third);

    state.cube = render::vao::Vao::new(render_api);
    state
        .cube
        .upload_v3(render_api, vec![first, second, third], 0);

    state.render_commands.push(RenderCommand::new_model(
        &state.cube,
        &state.basic_shader,
        vec![0, 1, 2],
        &state.camera,
    ));

    let cam_speed = 0.01;
    if input.keyboard[ASCII_A].pressing {
        state.camera.transform.position.x = state.camera.transform.position.x - cam_speed;
    }
    if input.keyboard[ASCII_D].pressing {
        state.camera.transform.position.x = state.camera.transform.position.x + cam_speed;
    }
    if input.keyboard[ASCII_S].pressing {
        state.camera.transform.position.y = state.camera.transform.position.y - cam_speed;
    }
    if input.keyboard[ASCII_W].pressing {
        state.camera.transform.position.y = state.camera.transform.position.y + cam_speed;
    }
}

pub fn engine_frame_end(state: &mut State) {
    state.camera.update_matricies();
}
