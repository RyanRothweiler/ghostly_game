// Get methods needed for the RenderApi from windows

#![allow(
    unused_variables,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_imports
)]

use gengar_render_opengl::ogl_render::*;

use libc;

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

// Maybe can remove these globals using a boxed closure?
// https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust/27832320#27832320

type func_glCreateShader = extern "stdcall" fn(i32) -> i32;
static mut extern_global_glCreateShader: Option<func_glCreateShader> = None;

// #[repr(C)]
#[allow(improper_ctypes_definitions)]
type func_glShaderSource = extern "stdcall" fn(u32, i32, *const *const libc::c_char, *const i32);
static mut extern_global_glShaderSource: Option<func_glShaderSource> = None;

pub fn get_render_api() -> RenderApi {
    unsafe { extern_global_glCreateShader = Some(wgl_get_proc_address!(s!("glCreateShader"))) };
    unsafe { extern_global_glShaderSource = Some(wgl_get_proc_address!(s!("glShaderSource"))) };

    RenderApi {
        gl_clear_color: glClearColor,
        gl_clear: clear,
        gl_compile_shader: gl_compile_shader,
        gl_create_shader: gl_create_shader,
        gl_shader_source: gl_shader_source,
    }
}

fn gl_shader_source(id: i32, shader_source: &str) {
    let shader_source_c = std::ffi::CString::new(shader_source).unwrap();

    unsafe {
        (extern_global_glShaderSource.unwrap())(
            id as u32,
            1,
            &shader_source_c.as_ptr(),
            std::ptr::null(),
        );
    }
}

fn gl_create_shader(x: i32) -> i32 {
    unsafe { (extern_global_glCreateShader.unwrap())(x) }
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
