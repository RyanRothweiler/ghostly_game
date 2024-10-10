// Get methods needed for the RenderApi from windows

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code
)]

use gengar_engine::{matricies::matrix_four_four::*, render::image::Image, vectors::*};
use gengar_render_opengl::*;

use libc;

use windows::core::*;
use windows::Win32::Graphics::OpenGL::*;

const GL_FLOAT: i32 = 0x1406;
const GL_UNSIGNED_INT: i32 = 0x1405;

#[macro_export]
macro_rules! wgl_get_proc_address {
    ($x:expr) => {{
        let func = wglGetProcAddress($x).unwrap();
        std::mem::transmute(func)
    }};
}

type func_glCreateShader = extern "stdcall" fn(i32) -> u32;
type func_glShaderSource = extern "stdcall" fn(u32, i32, *const *const libc::c_char, *const i32);
type func_glCompileShader = extern "stdcall" fn(u32);
type func_glGetShaderiv = extern "stdcall" fn(u32, i32, *mut i32);
type func_glShaderInfoLog = extern "stdcall" fn(u32, i32, *mut i32, *mut u8);
type func_glCreateProgram = extern "stdcall" fn() -> u32;
type func_glAttachShader = extern "stdcall" fn(u32, u32);
type func_glLinkProgram = extern "stdcall" fn(u32);
type func_glGenVertexArrays = extern "stdcall" fn(i32, *mut u32);
type func_glBindVertexArray = extern "stdcall" fn(u32);
type func_glGenBuffers = extern "stdcall" fn(i32, *mut u32);
type func_glBindBuffer = extern "stdcall" fn(i32, u32);
type func_glBufferData = extern "stdcall" fn(i32, i32, *const libc::c_void, i32);
type func_glUseProgram = extern "stdcall" fn(u32);
type func_glDrawElements = extern "stdcall" fn(i32, i32, i32, *const libc::c_void);
type func_glDrawArrays = extern "stdcall" fn(i32, i32, i32);
type func_glEnableVertexAttribArray = extern "stdcall" fn(u32);
type func_glGenTextures = extern "stdcall" fn(i32, *mut u32);
type func_glBindTexture = extern "stdcall" fn(i32, u32);
type func_glActiveTexture = extern "stdcall" fn(i32);
type func_glVertexAttribPointer =
    extern "stdcall" fn(u32, u32, i32, bool, i32, *const libc::c_void);

type func_glGetUniformLocation = extern "stdcall" fn(u32, *const libc::c_char) -> i32;
type func_glUniform1f = extern "stdcall" fn(i32, f32);
type func_glUniform1i = extern "stdcall" fn(i32, i32);
type func_glUniform3fv = extern "stdcall" fn(i32, i32, *const f32);
type func_glUniform4fv = extern "stdcall" fn(i32, i32, *const f32);
type func_glUniformMatrix4fv = extern "stdcall" fn(i32, i32, bool, *const f32);

pub struct WglMethods {
    glActiveTexture: func_glActiveTexture,
    glBindTexture: func_glBindTexture,
    glGenTextures: func_glGenTextures,
    glEnableVertexAttribArray: func_glEnableVertexAttribArray,
    glDrawArrays: func_glDrawArrays,
    glDrawElements: func_glDrawElements,
    glUseProgram: func_glUseProgram,
    glVertexAttribPointer: func_glVertexAttribPointer,
    glBufferData: func_glBufferData,
    glBindBuffer: func_glBindBuffer,
    glGenBuffers: func_glGenBuffers,
    glBindVertexArray: func_glBindVertexArray,
    glGenVertexArrays: func_glGenVertexArrays,
    glLinkProgram: func_glLinkProgram,
    glAttachShader: func_glAttachShader,
    glCreateProgram: func_glCreateProgram,
    glShaderInfoLog: func_glShaderInfoLog,
    glGetShaderiv: func_glGetShaderiv,
    glCompileShader: func_glCompileShader,
    glShaderSource: func_glShaderSource,
    glCreateShader: func_glCreateShader,

    glGetUniformLocation: func_glGetUniformLocation,
    glUniform1f: func_glUniform1f,
    glUniform1i: func_glUniform1i,
    glUniform3fv: func_glUniform3fv,
    glUniform4fv: func_glUniform4fv,
    glUniformMatrix4fv: func_glUniformMatrix4fv,
    // pub ogl_render_api: OglRenderApi,
}

impl gengar_render_opengl::OGLPlatformImpl for WglMethods {
    fn create_shader(&self, id: i32) -> u32 {
        return (self.glCreateShader)(id);
    }

    fn shader_source(&self, id: u32, source: &str) {
        let shader_source_c = std::ffi::CString::new(source).unwrap();
        (self.glShaderSource)(id, 1, &shader_source_c.as_ptr(), std::ptr::null());
    }

    fn compile_shader(&self, id: u32) {
        (self.glCompileShader)(id);
    }

    fn get_shader_iv(&self, id: u32, info_type: i32, output: *mut i32) {
        (self.glGetShaderiv)(id, info_type, output);
    }

    fn shader_info_log(
        &self,
        shader_id: u32,
        max_length: i32,
        output_length: *mut i32,
        output_buffer: &mut Vec<u8>,
    ) {
        (self.glShaderInfoLog)(
            shader_id,
            max_length,
            output_length,
            output_buffer.as_mut_ptr(),
        );
    }

    fn create_program(&self) -> u32 {
        return (self.glCreateProgram)();
    }

    fn attach_shader(&self, prog_id: u32, shader_id: u32) {
        (self.glAttachShader)(prog_id, shader_id);
    }

    fn link_program(&self, prog_id: u32) {
        (self.glLinkProgram)(prog_id);
    }

    fn gen_vertex_arrays(&self, count: i32, vao: *mut u32) {
        (self.glGenVertexArrays)(count, vao);
    }

    fn bind_vertex_array(&self, vao_id: u32) {
        (self.glBindVertexArray)(vao_id);
    }

    fn gen_buffers(&self, count: i32, buffers: *mut u32) {
        (self.glGenBuffers)(count, buffers);
    }

    fn bind_buffer(&self, typ: i32, buf_id: u32) {
        (self.glBindBuffer)(typ, buf_id);
    }

    fn buffer_data_v3(&self, buf_id: i32, data: &Vec<VecThreeFloat>, usage: i32) {
        let mut list_c: Vec<VecThreeFloatC> = data
            .into_iter()
            .map(|input| VecThreeFloatC::from(input))
            .collect();
        let ptr = list_c.as_mut_ptr() as *mut libc::c_void;
        let size: usize = std::mem::size_of::<VecThreeFloatC>() * list_c.len();
        (self.glBufferData)(buf_id, i32::try_from(size).unwrap(), ptr, usage);
    }

    fn buffer_data_v2(&self, buf_id: i32, data: &Vec<VecTwo>, usage: i32) {
        let mut list_c: Vec<VecTwoC> = data.into_iter().map(|input| VecTwoC::from(input)).collect();
        let ptr = list_c.as_mut_ptr() as *mut libc::c_void;
        let size: usize = std::mem::size_of::<VecTwoC>() * list_c.len();
        (self.glBufferData)(buf_id, i32::try_from(size).unwrap(), ptr, usage);
    }

    fn buffer_data_u32(&self, buf_id: i32, data: &Vec<u32>, usage: i32) {
        let mut list_c: Vec<u32> = data.clone();

        let ptr = list_c.as_mut_ptr() as *mut libc::c_void;
        let size: usize = std::mem::size_of::<u32>() * data.len();

        (self.glBufferData)(buf_id, i32::try_from(size).unwrap(), ptr, usage);
    }

    fn enable_vertex_attrib_array(&self, location: u32) {
        (self.glEnableVertexAttribArray)(location);
    }

    fn vertex_attrib_pointer_v3(&self, location: u32) {
        let stride: usize = std::mem::size_of::<VecThreeFloatC>();
        let stride: i32 = i32::try_from(stride).unwrap();

        (self.glVertexAttribPointer)(location, 3, GL_FLOAT, false, stride, std::ptr::null());
    }

    fn vertex_attrib_pointer_v2(&self, location: u32) {
        let stride: usize = std::mem::size_of::<VecTwoC>();
        let stride: i32 = i32::try_from(stride).unwrap();

        (self.glVertexAttribPointer)(location, 2, GL_FLOAT, false, stride, std::ptr::null());
    }

    fn gen_textures(&self, count: i32, id: *mut u32) {
        (self.glGenTextures)(count, id);
    }

    fn bind_texture(&self, typ: i32, id: u32) {
        (self.glBindTexture)(typ, id);
    }

    fn tex_parameter_i(&self, target: u32, pname: u32, param: i32) {
        unsafe {
            glTexParameteri(target, pname, param);
        }
    }

    fn tex_image_2d(
        &self,
        target: u32,
        gl_storage_format: i32,
        image_format: u32,
        image_pixel_format: u32,
        image: &Image,
    ) {
        let mip_level: i32 = 0;
        let border = 0;
        let data_ptr = image.data.as_ptr() as *const libc::c_void;

        unsafe {
            glTexImage2D(
                target,
                mip_level,
                gl_storage_format,
                image.width as i32,
                image.height as i32,
                border,
                image_format,
                image_pixel_format,
                data_ptr,
            );
        }
    }

    fn enable(&self, feature: u32) {
        unsafe {
            glEnable(feature);
        }
    }

    fn depth_func(&self, func: u32) {
        unsafe {
            glDepthFunc(func);
        }
    }

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { glClearColor(r, g, b, a) };
    }

    fn clear(&self) {
        unsafe { glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT) };
    }

    fn use_program(&self, prog_id: u32) {
        (self.glUseProgram)(prog_id);
    }

    fn get_uniform_location(&self, prog_id: u32, uniform_name: &str) -> i32 {
        let name_c = std::ffi::CString::new(uniform_name).unwrap();
        return (self.glGetUniformLocation)(prog_id, name_c.as_ptr());
    }

    fn uniform_matrix_4fv(&self, loc: i32, count: i32, transpose: bool, data: &M44) {
        let mut elems: [f32; 16] = [0.0; 16];
        for i in 0..data.elements.len() {
            elems[i] = data.elements[i] as f32;
        }

        (self.glUniformMatrix4fv)(loc, count, transpose, &elems[0]);
    }

    fn uniform_4fv(&self, loc: i32, count: i32, data: &VecFour) {
        let elems: [f32; 4] = [data.x as f32, data.y as f32, data.z as f32, data.w as f32];
        (self.glUniform4fv)(loc, count, &elems[0]);
    }

    fn uniform_3fv(&self, loc: i32, count: i32, data: &VecThreeFloat) {
        let elems: [f32; 3] = [data.x as f32, data.y as f32, data.z as f32];
        (self.glUniform3fv)(loc, count, &elems[0]);
    }

    fn uniform_1f(&self, loc: i32, data: f32) {
        (self.glUniform1f)(loc, data);
    }

    fn uniform_1i(&self, loc: i32, data: i32) {
        (self.glUniform1i)(loc, data);
    }

    fn active_texture(&self, id: i32) {
        (self.glActiveTexture)(id);
    }

    fn draw_elements(&self, mode: i32, indecies: &Vec<u32>) {
        let ptr = indecies.as_ptr() as *const libc::c_void;
        (self.glDrawElements)(mode, indecies.len() as i32, GL_UNSIGNED_INT, ptr);
    }
}

static mut extern_global_wgl_methods: Option<WglMethods> = None;

pub fn get_ogl_render_api() -> OglRenderApi {
    let wgl_methods = unsafe {
        WglMethods {
            glActiveTexture: wgl_get_proc_address!(s!("glActiveTexture")),
            glBindTexture: wgl_get_proc_address!(s!("glBindTexture")),
            glGenTextures: wgl_get_proc_address!(s!("glGenTextures")),
            glEnableVertexAttribArray: wgl_get_proc_address!(s!("glEnableVertexAttribArray")),
            glDrawArrays: wgl_get_proc_address!(s!("glDrawArrays")),
            glDrawElements: wgl_get_proc_address!(s!("glDrawElements")),
            glUseProgram: wgl_get_proc_address!(s!("glUseProgram")),
            glVertexAttribPointer: wgl_get_proc_address!(s!("glVertexAttribPointer")),
            glBufferData: wgl_get_proc_address!(s!("glBufferData")),
            glBindBuffer: wgl_get_proc_address!(s!("glBindBuffer")),
            glGenBuffers: wgl_get_proc_address!(s!("glGenBuffers")),
            glBindVertexArray: wgl_get_proc_address!(s!("glBindVertexArray")),
            glGenVertexArrays: wgl_get_proc_address!(s!("glGenVertexArrays")),
            glLinkProgram: wgl_get_proc_address!(s!("glLinkProgram")),
            glAttachShader: wgl_get_proc_address!(s!("glAttachShader")),
            glCreateProgram: wgl_get_proc_address!(s!("glCreateProgram")),
            glShaderInfoLog: wgl_get_proc_address!(s!("glGetShaderInfoLog")),
            glGetShaderiv: wgl_get_proc_address!(s!("glGetShaderiv")),
            glCompileShader: wgl_get_proc_address!(s!("glCompileShader")),
            glShaderSource: wgl_get_proc_address!(s!("glShaderSource")),
            glCreateShader: wgl_get_proc_address!(s!("glCreateShader")),

            glGetUniformLocation: wgl_get_proc_address!(s!("glGetUniformLocation")),
            glUniform1f: wgl_get_proc_address!(s!("glUniform1f")),
            glUniform1i: wgl_get_proc_address!(s!("glUniform1i")),
            glUniform3fv: wgl_get_proc_address!(s!("glUniform3fv")),
            glUniform4fv: wgl_get_proc_address!(s!("glUniform4fv")),
            glUniformMatrix4fv: wgl_get_proc_address!(s!("glUniformMatrix4fv")),
        }
    };

    let ogl_api = OglRenderApi {
        platform_api: Box::new(wgl_methods),
    };

    return ogl_api;
}
