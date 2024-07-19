#![allow(unused_variables)]

use std::include_str;

pub mod color;
pub mod error;
// pub mod state;

// Platform needs to provide these things
pub struct PlatformApi {
    pub gl_get_proc_address: fn(&str),
}

// Rendering system needs to provide these
/*
pub struct RenderApi {
    clear
}
*/

pub fn load_resources() {
    let basic_shader_frag = include_str!("../engine_resources/shaders/basic.fs");
    let basic_shader_vert = include_str!("../engine_resources/shaders/basic.vs");
}

pub fn engine_loop() {}
