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

#[allow(improper_ctypes_definitions)]
type func_glShaderSource = extern "stdcall" fn(u32, i32, *const *const libc::c_char, *const i32);
static mut extern_global_glShaderSource: Option<func_glShaderSource> = None;

type func_glCompileShader = extern "stdcall" fn(i32);
static mut extern_global_glCompileShader: Option<func_glCompileShader> = None;

type func_glGetShaderiv = extern "stdcall" fn(i32, i32, *mut i32);
static mut extern_global_glGetShaderiv: Option<func_glGetShaderiv> = None;

type func_glShaderInfoLog = extern "stdcall" fn(i32, i32, *mut i32, *mut u8);
static mut extern_global_glShaderInfoLog: Option<func_glShaderInfoLog> = None;

pub fn get_render_api() -> RenderApi {
    unsafe { extern_global_glCreateShader = Some(wgl_get_proc_address!(s!("glCreateShader"))) };
    unsafe { extern_global_glShaderSource = Some(wgl_get_proc_address!(s!("glShaderSource"))) };
    unsafe { extern_global_glCompileShader = Some(wgl_get_proc_address!(s!("glCompileShader"))) };
    unsafe { extern_global_glGetShaderiv = Some(wgl_get_proc_address!(s!("glGetShaderiv"))) };
    unsafe {
        extern_global_glShaderInfoLog = Some(wgl_get_proc_address!(s!("glGetShaderInfoLog")))
    };

    RenderApi {
        gl_clear_color: gl_clear_color,
        gl_clear: clear,
        gl_compile_shader: gl_compile_shader,
        gl_create_shader: gl_create_shader,
        gl_shader_source: gl_shader_source,
        gl_get_shader_iv: gl_get_shader_iv,
        gl_shader_info_log: gl_shader_info_log,
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

fn gl_compile_shader(shader_id: i32) {
    unsafe { (extern_global_glCompileShader.unwrap())(shader_id) }
}

fn gl_get_shader_iv(shader_id: i32, info_type: i32, output: *mut i32) {
    unsafe { (extern_global_glGetShaderiv.unwrap())(shader_id, info_type, output) }
}

fn gl_shader_info_log(
    shader_id: i32,
    max_length: i32,
    output_length: *mut i32,
    output_buffer: &mut Vec<u8>,
) {
    unsafe {
        (extern_global_glShaderInfoLog.unwrap())(
            shader_id,
            max_length,
            output_length,
            output_buffer.as_mut_ptr(),
        )
    }
}

fn gl_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe { OpenGL::glClearColor(r, g, b, a) };
}

pub fn clear() {
    unsafe { OpenGL::glClear(GL_COLOR_BUFFER_BIT) };
}
