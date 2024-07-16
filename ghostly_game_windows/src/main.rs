#![allow(
    unused_imports,
    unused_variables,
    unused_unsafe,
    dead_code,
    unused_mut,
    unused_assignments
)]

// windows hello triangle in rust
// https://rust-tutorials.github.io/triangle-from-scratch/loading_opengl/win32.html

// example of entire setup
// https://github.com/glowcoil/raw-gl-context/blob/master/src/win.rs

use gengar_engine::engine;
use gengar_render_opengl::render;

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
    extern "stdcall" fn(HDC, *const i32, *const f32, u8, *const i32, *const i32) -> bool;
//extern "stdcall" fn(*const c_void, *const u8, *const u8, u32) -> i32;

//HDC hdc, const int *piAttribIList, const FLOAT *pfAttribFList, UINT nMaxFormats, int *piFormats, UINT *nNumFormats

fn gl_get_proc_address(proc: &str) {}

fn main() {
    let mut platform_api = engine::PlatformApi {
        gl_get_proc_address: gl_get_proc_address,
    };

    let mut render_api = render::RenderApi {
        clear: gl::gl_clear,
    };

    unsafe {
        let instance = GetModuleHandleA(None).unwrap();

        let window_class: PCSTR = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(windows_callback),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        // create main window
        let main_window_handle = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
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

        // Use dummy device context to get the proc addresses needed for the final window
        {
            let dummy_window_class: PCSTR = s!("dummy_window");

            let dummy_wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                hInstance: instance.into(),
                lpszClassName: dummy_window_class,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(dummy_windows_callback),
                ..Default::default()
            };

            let dummy_atom = RegisterClassA(&dummy_wc);
            debug_assert!(dummy_atom != 0);

            let dummy_win_handle = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                dummy_window_class,
                s!("ghostly_dummy"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
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
                dwFlags: PFD_SUPPORT_OPENGL | PFD_DRAW_TO_WINDOW,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cDepthBits: 24,
                cStencilBits: 8,

                ..Default::default()
            };

            let result: i32 = ChoosePixelFormat(dummy_device_context, &dummy_desired_pixel_format);
            if result == 0 {
                println!("Invalid pixel format");
            }

            let suggested_pixel_format: PIXELFORMATDESCRIPTOR = PIXELFORMATDESCRIPTOR {
                ..Default::default()
            };
            let suggested_pixel_format_index: i32 =
                ChoosePixelFormat(dummy_device_context, &suggested_pixel_format);
            if suggested_pixel_format_index == 0 {
                println!("Invalid suggested pixel format");
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

            //wglDeleteContext(DummyOpenGLRC);
            ReleaseDC(dummy_win_handle, dummy_device_context);
            DestroyWindow(dummy_win_handle).unwrap();
        }

        // init opengl
        let device_context = GetDC(main_window_handle);

        // setup real opengl window
        /*
        #[rustfmt::skip]
        let pixel_format_attribs: [i32; 15] = [
            gl::WGL_DRAW_TO_WINDOW_ARB,
            gl::GL_TRUE,

            gl::WGL_SUPPORT_OPENGL_ARB,
            gl::GL_TRUE,

            gl::WGL_DOUBLE_BUFFER_ARB,
            gl::GL_TRUE,

            gl::WGL_PIXEL_TYPE_ARB,
            gl::WGL_TYPE_RGBA_ARB,

            gl::WGL_COLOR_BITS_ARB,
            32,

            gl::WGL_DEPTH_BITS_ARB,
            24,

            gl::WGL_STENCIL_BITS_ARB,
            8,

            0,
        ];
        let mut extend_pick: i32 = 0;
        let mut suggested_pixel_format_index: i32 = 0;
        let res = (wgl_choose_pixel_format_arb.unwrap())(
            device_context,
            &pixel_format_attribs[0],
            0,
            1,
            &mut extend_pick,
            &mut suggested_pixel_format_index,
        );
        */

        let mut message = MSG::default();

        render::setup(&platform_api);

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);

            let time_start: SystemTime = SystemTime::now();

            engine::engine_loop();
            game::game_loop();
            render::render(&render_api);

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
