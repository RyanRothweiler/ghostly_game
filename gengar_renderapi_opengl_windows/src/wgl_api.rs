// Get methods needed for the RenderApi from windows

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use gengar_engine::engine::matricies::matrix_four_four::*;
use gengar_engine::engine::vectors::*;
use gengar_render_opengl::ogl_render::*;

use libc;

use windows::core::*;
use windows::Win32::Graphics::OpenGL::*;
use windows::Win32::Graphics::*;

const GL_FLOAT: i32 = 0x1406;
const GL_UNSIGNED_INT: i32 = 0x1405;

#[macro_export]
macro_rules! wgl_get_proc_address {
    ($x:expr) => {{
        let func = wglGetProcAddress($x).unwrap();
        std::mem::transmute(func)
    }};
}

// Maybe can remove these globals using a boxed closure?
// https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust/27832320#27832320

type func_glCreateShader = extern "stdcall" fn(i32) -> u32;
static mut extern_global_glCreateShader: Option<func_glCreateShader> = None;

type func_glShaderSource = extern "stdcall" fn(u32, i32, *const *const libc::c_char, *const i32);
static mut extern_global_glShaderSource: Option<func_glShaderSource> = None;

type func_glCompileShader = extern "stdcall" fn(u32);
static mut extern_global_glCompileShader: Option<func_glCompileShader> = None;

type func_glGetShaderiv = extern "stdcall" fn(u32, i32, *mut i32);
static mut extern_global_glGetShaderiv: Option<func_glGetShaderiv> = None;

type func_glShaderInfoLog = extern "stdcall" fn(u32, i32, *mut i32, *mut u8);
static mut extern_global_glShaderInfoLog: Option<func_glShaderInfoLog> = None;

type func_glCreateProgram = extern "stdcall" fn() -> u32;
static mut extern_global_glCreateProgram: Option<func_glCreateProgram> = None;

type func_glAttachShader = extern "stdcall" fn(u32, u32);
static mut extern_global_glAttachShader: Option<func_glAttachShader> = None;

type func_glLinkProgram = extern "stdcall" fn(u32);
static mut extern_global_glLinkProgram: Option<func_glLinkProgram> = None;

type func_glGenVertexArrays = extern "stdcall" fn(i32, *mut u32);
static mut extern_global_glGenVertexArrays: Option<func_glGenVertexArrays> = None;

type func_glBindVertexArray = extern "stdcall" fn(u32);
static mut extern_global_glBindVertexArray: Option<func_glBindVertexArray> = None;

type func_glGenBuffers = extern "stdcall" fn(i32, *mut u32);
static mut extern_global_glGenBuffers: Option<func_glGenBuffers> = None;

type func_glBindBuffer = extern "stdcall" fn(i32, u32);
static mut extern_global_glBindBuffer: Option<func_glBindBuffer> = None;

type func_glBufferData = extern "stdcall" fn(i32, i32, *const libc::c_void, i32);
static mut extern_global_glBufferData: Option<func_glBufferData> = None;

type func_glVertexAttribPointer =
    extern "stdcall" fn(u32, u32, i32, bool, i32, *const libc::c_void);
static mut extern_global_glVertexAttribPointer: Option<func_glVertexAttribPointer> = None;

type func_glUseProgram = extern "stdcall" fn(u32);
static mut extern_global_glUseProgram: Option<func_glUseProgram> = None;

type func_glDrawElements = extern "stdcall" fn(i32, i32, i32, *const libc::c_void);
static mut extern_global_glDrawElements: Option<func_glDrawElements> = None;

type func_glDrawArrays = extern "stdcall" fn(i32, i32, i32);
static mut extern_global_glDrawArrays: Option<func_glDrawArrays> = None;

type func_glEnableVertexAttribArray = extern "stdcall" fn(u32);
static mut extern_global_glEnableVertexAttribArray: Option<func_glEnableVertexAttribArray> = None;

type func_glGetUniformLocation = extern "stdcall" fn(u32, *const libc::c_char) -> i32;
static mut extern_global_glGetUniformLocation: Option<func_glGetUniformLocation> = None;

type func_glUniformMatrix4fv = extern "stdcall" fn(i32, i32, bool, *const f32);
static mut extern_global_glUniformMatrix4fv: Option<func_glUniformMatrix4fv> = None;

pub fn get_ogl_render_api() -> OglRenderApi {
    unsafe {
        extern_global_glCreateShader = Some(wgl_get_proc_address!(s!("glCreateShader")));
        extern_global_glShaderSource = Some(wgl_get_proc_address!(s!("glShaderSource")));
        extern_global_glCompileShader = Some(wgl_get_proc_address!(s!("glCompileShader")));
        extern_global_glGetShaderiv = Some(wgl_get_proc_address!(s!("glGetShaderiv")));
        extern_global_glCreateProgram = Some(wgl_get_proc_address!(s!("glCreateProgram")));
        extern_global_glAttachShader = Some(wgl_get_proc_address!(s!("glAttachShader")));
        extern_global_glLinkProgram = Some(wgl_get_proc_address!(s!("glLinkProgram")));
        extern_global_glGenBuffers = Some(wgl_get_proc_address!(s!("glGenBuffers")));
        extern_global_glBindBuffer = Some(wgl_get_proc_address!(s!("glBindBuffer")));
        extern_global_glBufferData = Some(wgl_get_proc_address!(s!("glBufferData")));
        extern_global_glBindVertexArray = Some(wgl_get_proc_address!(s!("glBindVertexArray")));
        extern_global_glGenVertexArrays = Some(wgl_get_proc_address!(s!("glGenVertexArrays")));
        extern_global_glShaderInfoLog = Some(wgl_get_proc_address!(s!("glGetShaderInfoLog")));
        extern_global_glUseProgram = Some(wgl_get_proc_address!(s!("glUseProgram")));
        extern_global_glDrawElements = Some(wgl_get_proc_address!(s!("glDrawElements")));
        extern_global_glDrawArrays = Some(wgl_get_proc_address!(s!("glDrawArrays")));
        extern_global_glUniformMatrix4fv = Some(wgl_get_proc_address!(s!("glUniformMatrix4fv")));
        extern_global_glGetUniformLocation =
            Some(wgl_get_proc_address!(s!("glGetUniformLocation")));
        extern_global_glEnableVertexAttribArray =
            Some(wgl_get_proc_address!(s!("glEnableVertexAttribArray")));
        extern_global_glVertexAttribPointer =
            Some(wgl_get_proc_address!(s!("glVertexAttribPointer")));
    }

    OglRenderApi {
        gl_clear_color: gl_clear_color,
        gl_clear: clear,
        gl_compile_shader: gl_compile_shader,
        gl_create_shader: gl_create_shader,
        gl_shader_source: gl_shader_source,
        gl_get_shader_iv: gl_get_shader_iv,
        gl_shader_info_log: gl_shader_info_log,
        gl_create_program: gl_create_program,
        gl_attach_shader: gl_attach_shader,
        gl_link_program: gl_link_program,
        gl_gen_vertex_arrays: gl_gen_vertex_arrays,
        gl_bind_vertex_array: gl_bind_vertex_array,
        gl_gen_buffers: gl_gen_buffers,
        gl_bind_buffer: gl_bind_buffer,

        gl_buffer_data_v3: gl_buffer_data_v3,
        gl_buffer_data_u32: gl_buffer_data_u32,

        gl_vertex_attrib_pointer_v3: gl_vertex_attrib_pointer_v3,
        gl_use_program: gl_use_program,
        gl_draw_elements: gl_draw_elements,
        gl_enable_vertex_attrib_array: gl_enable_vertex_attrib_array,
        gl_get_uniform_location: gl_get_uniform_location,
        gl_uniform_matrix_4fv: gl_uniform_matrix_4fv,
    }
}

fn gl_shader_source(id: u32, shader_source: &str) {
    let shader_source_c = std::ffi::CString::new(shader_source).unwrap();

    unsafe {
        (extern_global_glShaderSource.unwrap())(id, 1, &shader_source_c.as_ptr(), std::ptr::null());
    }
}

fn gl_use_program(id: u32) {
    unsafe { (extern_global_glUseProgram.unwrap())(id) }
}

fn gl_create_shader(ty: i32) -> u32 {
    unsafe { (extern_global_glCreateShader.unwrap())(ty) }
}

fn gl_enable_vertex_attrib_array(loc: u32) {
    unsafe { (extern_global_glEnableVertexAttribArray.unwrap())(loc) }
}

fn gl_buffer_data_v3(target: i32, data: &Vec<VecThreeFloat>, usage: i32) {
    let mut list_c: Vec<VecThreeFloatC> = data
        .into_iter()
        .map(|input| VecThreeFloatC::from(input))
        .collect();
    let ptr = list_c.as_mut_ptr() as *mut libc::c_void;
    let size: usize = std::mem::size_of::<VecThreeFloatC>() * list_c.len();
    unsafe {
        (extern_global_glBufferData.unwrap())(target, i32::try_from(size).unwrap(), ptr, usage)
    }
}

fn gl_buffer_data_u32(target: i32, data: &Vec<u32>, usage: i32) {
    // let mut list_c: Vec<u32> = data.into_iter().map(|input| *input as u64).collect();
    let mut list_c: Vec<u32> = data.clone();

    let ptr = list_c.as_mut_ptr() as *mut libc::c_void;
    let size: usize = std::mem::size_of::<u32>() * data.len();

    unsafe {
        (extern_global_glBufferData.unwrap())(target, i32::try_from(size).unwrap(), ptr, usage)
    }
}

fn gl_draw_elements(mode: i32, indecies: &Vec<u32>) {
    let ptr = indecies.as_ptr() as *const libc::c_void;
    unsafe {
        (extern_global_glDrawElements.unwrap())(mode, indecies.len() as i32, GL_UNSIGNED_INT, ptr)
    }
}

fn gl_compile_shader(shader_id: u32) {
    unsafe { (extern_global_glCompileShader.unwrap())(shader_id) }
}

fn gl_get_shader_iv(shader_id: u32, info_type: i32, output: *mut i32) {
    unsafe { (extern_global_glGetShaderiv.unwrap())(shader_id, info_type, output) }
}

fn gl_bind_buffer(ty: i32, buf_id: u32) {
    unsafe { (extern_global_glBindBuffer.unwrap())(ty, buf_id) }
}

fn gl_create_program() -> u32 {
    unsafe { (extern_global_glCreateProgram.unwrap())() }
}

fn gl_gen_vertex_arrays(count: i32, vao: *mut u32) {
    unsafe { (extern_global_glGenVertexArrays.unwrap())(count, vao) }
}

fn gl_link_program(prog_id: u32) {
    unsafe { (extern_global_glLinkProgram.unwrap())(prog_id) }
}

fn gl_vertex_attrib_pointer_v3(location: u32) {
    let stride: usize = std::mem::size_of::<VecThreeFloatC>();
    let stride: i32 = i32::try_from(stride).unwrap();

    unsafe {
        (extern_global_glVertexAttribPointer.unwrap())(
            location,
            3,
            GL_FLOAT,
            false,
            stride,
            std::ptr::null(),
        )
    }
}

fn gl_attach_shader(prog_id: u32, shader_id: u32) {
    unsafe { (extern_global_glAttachShader.unwrap())(prog_id, shader_id) }
}

fn gl_bind_vertex_array(vao_id: u32) {
    unsafe { (extern_global_glBindVertexArray.unwrap())(vao_id) }
}

fn gl_gen_buffers(count: i32, buffers: *mut u32) {
    unsafe { (extern_global_glGenBuffers.unwrap())(count, buffers) }
}

fn gl_shader_info_log(
    shader_id: u32,
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

pub fn gl_get_uniform_location(prog_id: u32, uniform_name: &str) -> i32 {
    let name_c = std::ffi::CString::new(uniform_name).unwrap();
    unsafe { return (extern_global_glGetUniformLocation.unwrap())(prog_id, name_c.as_ptr()) };
}

pub fn gl_uniform_matrix_4fv(loc: i32, count: i32, transpose: bool, mat: &M44) {
    unsafe {
        let mut elems: [f32; 16] = [0.0; 16];
        for i in 0..mat.elements.len() {
            elems[i] = mat.elements[i] as f32;
        }
        (extern_global_glUniformMatrix4fv.unwrap())(loc, count, transpose, &elems[0]);
    }
}
