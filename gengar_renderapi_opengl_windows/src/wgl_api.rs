#![allow(unused_imports, unused_variables, non_snake_case)]

use gengar_engine::engine::*;
use gengar_render_opengl::ogl_render::*;

use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::Graphics::*;

pub fn get_render_api() -> RenderApi {
    RenderApi {
        glClearColor: glClearColor,
        glClear: clear,
    }
}

fn glClearColor(r: f32, g: f32, b: f32, a: f32) {
    unsafe { OpenGL::glClearColor(r, g, b, a) };
}

pub fn clear() {
    unsafe { OpenGL::glClear(GL_COLOR_BUFFER_BIT) };
}
