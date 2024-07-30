#![allow(unused_variables, unused_imports)]

use std::include_str;

pub mod color;
pub mod error;
pub mod vectors;

use vectors::*;

pub mod render;
// pub mod state;

// Platform needs to provide these things
/*
pub struct PlatformApi {
    pub gl_get_proc_address: fn(&str),
}
*/

pub fn load_resources(render_api: &impl render::RenderApi) {
    let basic_shader_frag = include_str!("../engine_resources/shaders/basic.fs");
    let basic_shader_vert = include_str!("../engine_resources/shaders/basic.vs");

    let prog_id = render_api
        .make_shader_program(basic_shader_vert, basic_shader_frag)
        .unwrap();
    println!("engine prog id {prog_id}");

    let mut cube_mesh: Vec<VecThreeFloat> = vec![];
    cube_mesh.push(VecThreeFloat::new(0.0, 0.0, 0.0));
    cube_mesh.push(VecThreeFloat::new(0.0, 10.0, 0.0));
    cube_mesh.push(VecThreeFloat::new(10.0, 0.0, 0.0));

    let mut cube = render::vao::Vao::new(render_api);
    cube.upload_v3(render_api, cube_mesh, 0);
}

pub fn engine_loop() {}
