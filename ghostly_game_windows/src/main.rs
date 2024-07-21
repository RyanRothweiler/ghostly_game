#![allow(unused_variables, dead_code, unused_assignments, unused_imports)]

// windows hello triangle in rust
// https://rust-tutorials.github.io/triangle-from-scratch/loading_opengl/win32.html

// example of entire setup
// https://github.com/glowcoil/raw-gl-context/blob/master/src/win.rs

use gengar_engine::engine;
use gengar_render_opengl::ogl_render::*;
use ghostly_game::game;

use windows::Win32::Graphics::Gdi::*;
use windows::Win32::Graphics::OpenGL::*;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

use std::thread;
use std::time::{Duration, SystemTime};

mod gl;

const FRAME_TARGET_FPS: f64 = 60.0;
const FRAME_TARGET: Duration = Duration::from_secs((1.0 / FRAME_TARGET_FPS) as u64);

type FuncWglChoosePixelFormatARB =
    extern "stdcall" fn(HDC, *const i32, *const f32, u32, *mut i32, *mut i32) -> i32;

type FuncWglCreateContextAttribsARB = extern "system" fn(HDC, i32, *const i32) -> HGLRC;

//extern "stdcall" fn(*const c_void, *const u8, *const u8, u32) -> i32;

//HDC hdc, const int *piAttribIList, const FLOAT *pfAttribFList, UINT nMaxFormats, int *piFormats, UINT *nNumFormats

struct EngienRenderApi {
    pub create_shader: fn() -> i32,
}

// static mut OGL_REND_API: Option<gengar_render_opengl::ogl_render::RenderApi> = None;

/*
pub fn create_shader() -> i32 {
    unsafe {
        ogl_
        // OGL_REND_API.unwrap().make_shader_program();
    }

    return 0;
}
*/

static mut RUNNING: bool = true;

fn main() {
    /*
    let platform_api = engine::PlatformApi {
        gl_get_proc_address: gl_get_proc_address,
    };
    */

    unsafe {
        let instance = GetModuleHandleA(None).unwrap();

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: instance.into(),
            lpszClassName: s!("main_window_class"),
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(windows_callback),
            ..Default::default()
        };

        let result = RegisterClassA(&wc);
        if result == 0 {
            eprintln!("Error register windows class");
            return;
        }

        // create main window
        let main_window_handle = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            wc.lpszClassName,
            s!("Ghostly"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            2500,
            1300,
            None,
            None,
            instance,
            None,
        );

        // functions to get
        let mut wgl_choose_pixel_format_arb: Option<FuncWglChoosePixelFormatARB> = None;
        let mut wgl_create_context_attribs: Option<FuncWglCreateContextAttribsARB> = None;

        // Use dummy device context to get the proc addresses needed for the final window
        {
            let dummy_wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance.into(),
                lpszClassName: s!("dummy_window"),
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(dummy_windows_callback),
                ..Default::default()
            };

            let dummy_atom = RegisterClassA(&dummy_wc);
            debug_assert!(dummy_atom != 0);

            let dummy_win_handle = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                dummy_wc.lpszClassName,
                s!("ghostly_dummy"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                None,
            );

            let dummy_device_context = GetDC(dummy_win_handle);

            let nsize = std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16;
            let dummy_desired_pixel_format: PIXELFORMATDESCRIPTOR = PIXELFORMATDESCRIPTOR {
                nSize: nsize,
                nVersion: 1,
                iPixelType: PFD_TYPE_RGBA,
                dwFlags: PFD_SUPPORT_OPENGL | PFD_DRAW_TO_WINDOW | PFD_DOUBLEBUFFER,
                cColorBits: 32,
                cAlphaBits: 8,
                cDepthBits: 24,
                cStencilBits: 8,
                iLayerType: gl::PFD_MAIN_PLANE,
                ..Default::default()
            };

            let suggested_pixel_format_index: i32 =
                ChoosePixelFormat(dummy_device_context, &dummy_desired_pixel_format);
            if suggested_pixel_format_index == 0 {
                println!("Invalid pixel format");
            }

            SetPixelFormat(
                dummy_device_context,
                suggested_pixel_format_index,
                &dummy_desired_pixel_format,
            )
            .unwrap();

            let dummy_opengl_context = wglCreateContext(dummy_device_context).unwrap();
            wglMakeCurrent(dummy_device_context, dummy_opengl_context).unwrap();

            // get proc addresses
            let wgl_choose_pixel_format_arb_proc =
                wglGetProcAddress(s!("wglChoosePixelFormatARB")).unwrap();
            wgl_choose_pixel_format_arb =
                Some(std::mem::transmute(wgl_choose_pixel_format_arb_proc));

            let wgl_create_context_attribs_proc =
                wglGetProcAddress(s!("wglCreateContextAttribsARB")).unwrap();
            wgl_create_context_attribs = Some(std::mem::transmute(wgl_create_context_attribs_proc));

            wglDeleteContext(dummy_opengl_context).expect("error");
            wglMakeCurrent(
                dummy_device_context,
                windows::Win32::Graphics::OpenGL::HGLRC::default(),
            )
            .unwrap();
            ReleaseDC(dummy_win_handle, dummy_device_context);
            DestroyWindow(dummy_win_handle).unwrap();
        }

        // init opengl
        let device_context = GetDC(main_window_handle);

        // setup real opengl window
        #[rustfmt::skip]
        let pixel_format_attribs: [i32; 17] = [
            gl::WGL_DRAW_TO_WINDOW_ARB as i32,      1,
            gl::WGL_SUPPORT_OPENGL_ARB as i32,      1,
            gl::WGL_DOUBLE_BUFFER_ARB as i32,       1,
            gl::WGL_PIXEL_TYPE_ARB as i32,          gl::WGL_TYPE_RGBA_ARB as i32,
            gl::WGL_ACCELERATION_ARB as i32,        gl::WGL_FULL_ACCELERATION_ARB as i32,

            gl::WGL_COLOR_BITS_ARB as i32,          32,
            gl::WGL_DEPTH_BITS_ARB as i32,          24,
            gl::WGL_STENCIL_BITS_ARB as i32,        8,

            0,
        ];
        let mut extend_pick: i32 = 0;
        let mut suggested_pixel_format_index: i32 = 0;
        let res = (wgl_choose_pixel_format_arb.unwrap())(
            device_context,
            pixel_format_attribs.as_ptr(),
            std::ptr::null(),
            1,
            &mut suggested_pixel_format_index,
            &mut extend_pick,
        );

        let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
        DescribePixelFormat(
            device_context,
            suggested_pixel_format_index,
            std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32,
            Some(&mut pfd),
        );
        SetPixelFormat(device_context, suggested_pixel_format_index, &pfd).unwrap();

        #[rustfmt::skip]
        let context_attribs = [
            gl::WGL_CONTEXT_MAJOR_VERSION_ARB as i32, 3 as i32,
            gl::WGL_CONTEXT_MINOR_VERSION_ARB as i32, 3 as i32,
            gl::WGL_CONTEXT_PROFILE_MASK_ARB as i32, gl::WGL_CONTEXT_CORE_PROFILE_BIT_ARB as i32,
            0
        ];

        let wgl_context =
            wgl_create_context_attribs.unwrap()(device_context, 0, context_attribs.as_ptr());

        wglMakeCurrent(device_context, wgl_context).unwrap();

        // after context is setup, get the render api calls
        let render_api = gengar_renderapi_opengl_windows::wgl_api::get_render_api();
        // OGL_REND_API = Some(render_api);

        /*
        let engine_render_api = gengar_engine::engine::render::RenderApi {
            make_shader_program: gengar_render_opengl::ogl_render::pmake_shader_program,
        };
        */

        // engine::load_resources(&render_api);

        while RUNNING {
            let mut message = MSG::default();

            if GetMessageA(&mut message, None, 0, 0).into() {
                DispatchMessageA(&message);
            }

            let time_start: SystemTime = SystemTime::now();

            engine::engine_loop();
            game::game_loop();
            // render(&render_api);

            wglSwapLayerBuffers(device_context, gl::WGL_SWAP_MAIN_PLANE).unwrap();

            let time_end: SystemTime = SystemTime::now();
            let frame_duration: Duration = time_end.duration_since(time_start).unwrap();

            if FRAME_TARGET > frame_duration {
                let to_sleep: Duration = FRAME_TARGET - frame_duration;
                let slp = to_sleep.as_millis();
                thread::sleep(to_sleep);
            }
        }
    }
}

extern "system" fn windows_callback(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                _ = ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                RUNNING = false;

                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

extern "system" fn dummy_windows_callback(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcA(window, message, wparam, lparam) }
}
