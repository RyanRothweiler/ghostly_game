#![allow(unused_variables, unused_imports)]

pub mod color;

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

pub fn engine_loop() {}
