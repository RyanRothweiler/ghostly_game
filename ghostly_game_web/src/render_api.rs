use gengar_engine::engine::{
    error::*,
    matricies::matrix_four_four::*,
    render::{vao::Vao, RenderApi as EngineRenderApiTrait},
    vectors::*,
};
use gengar_render_opengl::ogl_render::*;

use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub static mut GL_CONTEXT: Option<WebGl2RenderingContext> = None;

/*
pub fn get_render_api() -> OglRenderApi {
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
        gl_vertex_attrib_pointer_v3: gl_vertex_attrib_pointer_v3,
        gl_use_program: gl_use_program,
        gl_draw_elements: gl_draw_elements,
        gl_enable_vertex_attrib_array: gl_enable_vertex_attrib_array,
        gl_get_uniform_location: gl_get_uniform_location,
        gl_uniform_matrix_4fv: gl_uniform_matrix_4fv,
    }
}

fn gl_shader_source(id: u32, shader_source: &str) {
    todo!()
}

fn gl_use_program(id: u32) {
    todo!()
}

fn gl_create_shader(ty: i32) -> u32 {
    unsafe {
        let v = (GL_CONTEXT.as_mut().unwrap())
            .create_shader(ty as u32)
            .unwrap();
        return 0;
    }
}

fn gl_enable_vertex_attrib_array(loc: u32) {
    todo!()
}

fn gl_buffer_data_v3(target: i32, data: &Vec<VecThreeFloat>, usage: i32) {
    todo!()
}

fn gl_draw_elements(mode: i32, indecies: &Vec<u32>) {
    todo!()
}

fn gl_compile_shader(shader_id: u32) {
    todo!()
}

fn gl_get_shader_iv(shader_id: u32, info_type: i32, output: *mut i32) {
    todo!()
}

fn gl_bind_buffer(ty: i32, buf_id: u32) {
    todo!()
}

fn gl_create_program() -> u32 {
    todo!()
}

fn gl_gen_vertex_arrays(count: i32, vao: *mut u32) {
    todo!()
}

fn gl_link_program(prog_id: u32) {
    todo!()
}

fn gl_vertex_attrib_pointer_v3(location: u32) {
    todo!()
}

fn gl_attach_shader(prog_id: u32, shader_id: u32) {
    todo!()
}

fn gl_bind_vertex_array(vao_id: u32) {
    todo!()
}

fn gl_gen_buffers(count: i32, buffers: *mut u32) {
    todo!()
}

fn gl_shader_info_log(
    shader_id: u32,
    max_length: i32,
    output_length: *mut i32,
    output_buffer: &mut Vec<u8>,
) {
    todo!()
}

fn gl_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).clear_color(r, g, b, a);
    }
}

pub fn clear() {
    unsafe {
        (GL_CONTEXT.as_mut().unwrap()).clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}

pub fn gl_get_uniform_location(prog_id: u32, uniform_name: &str) -> i32 {
    todo!()
}

pub fn gl_uniform_matrix_4fv(loc: i32, count: i32, transpose: bool, mat: &M44) {
    todo!()
}
*/
