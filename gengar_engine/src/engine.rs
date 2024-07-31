#![allow(unused_variables, unused_imports, non_upper_case_globals)]

use std::include_str;

pub mod color;
pub mod error;
pub mod matricies;
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

static mut global_cube: Option<u32> = None;
static mut global_prog: Option<u32> = None;

pub fn load_resources(render_api: &impl render::RenderApi) {
    let basic_shader_frag = include_str!("../engine_resources/shaders/basic.fs");
    let basic_shader_vert = include_str!("../engine_resources/shaders/basic.vs");

    let prog_id: u32 = render_api
        .make_shader_program(basic_shader_vert, basic_shader_frag)
        .unwrap();
    println!("engine prog id {prog_id}");

    let mut cube_mesh: Vec<VecThreeFloat> = vec![];
    cube_mesh.push(VecThreeFloat::new(-0.5, -0.5, 0.0));
    cube_mesh.push(VecThreeFloat::new(0.5, -0.5, 0.0));
    cube_mesh.push(VecThreeFloat::new(0.0, 0.5, 0.0));

    let mut cube = render::vao::Vao::new(render_api);
    cube.upload_v3(render_api, cube_mesh, 0);

    unsafe {
        global_cube = Some(cube.id);
        global_prog = Some(prog_id);
    }
}

pub fn engine_loop(render_api: &impl render::RenderApi) {
    unsafe {
        // cube.unwrap();
        let indecies: Vec<u32> = vec![0, 1, 2];
        render_api.render(global_prog.unwrap(), global_cube.unwrap(), &indecies);
    }
}
