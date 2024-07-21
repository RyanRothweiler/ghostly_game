#![allow(unused_variables, unused_imports)]

use std::include_str;

use gengar_render_opengl::ogl_render;

pub mod color;
pub mod error;

pub mod render;

// pub mod state;

// Platform needs to provide these things
pub struct PlatformApi {
    pub gl_get_proc_address: fn(&str),
}

pub fn load_resources(render_api: &gengar_render_opengl::ogl_render::RenderApi) {
    let basic_shader_frag = include_str!("../engine_resources/shaders/basic.fs");
    let basic_shader_vert = include_str!("../engine_resources/shaders/basic.vs");

    let prog_id = render_api.make_shader_program();
    println!("engine prog id {prog_id}");
}

pub fn engine_loop() {}
