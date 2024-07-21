// Get methods needed for the RenderApi from windows

#![allow(
    unused_variables,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]

use gengar_render_opengl::ogl_render::*;

use windows::core::*;
use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::Graphics::*;

#[macro_export]
macro_rules! wgl_get_proc_address {
    ($x:expr) => {{
        let func = wglGetProcAddress($x).unwrap();
        std::mem::transmute(func)
    }};
}

type func_glCreateShader = extern "stdcall" fn() -> i32;
static mut extern_global_glCreateShader: Option<func_glCreateShader> = None;

pub fn get_render_api() -> RenderApi {
    unsafe { extern_global_glCreateShader = Some(wgl_get_proc_address!(s!("glCreateShader"))) };

    RenderApi {
        glClearColor: glClearColor,
        glClear: clear,
        glCompileShader: gl_compile_shader,
        glCreateShader: gl_create_shader,
    }
}

fn foo(input: &str) {}

fn gl_create_shader() -> i32 {
    unsafe { (extern_global_glCreateShader.unwrap())() }
}

fn gl_compile_shader(shader_id: u32) {
    todo!();
}

fn glClearColor(r: f32, g: f32, b: f32, a: f32) {
    unsafe { OpenGL::glClearColor(r, g, b, a) };
}

pub fn clear() {
    unsafe { OpenGL::glClear(GL_COLOR_BUFFER_BIT) };
}
