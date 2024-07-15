#![allow(unused_imports, unused_variables, unused_unsafe, dead_code, unused_mut)]

use gengar_engine::engine;
use gengar_render_opengl::render;

use ghostly_game::game;

use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

use std::thread;
use std::time::{Duration, SystemTime};

mod gl;

const FRAME_TARGET_FPS: f64 = 60.0;
const FRAME_TARGET: Duration = Duration::from_secs((1.0 / FRAME_TARGET_FPS) as u64);

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
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
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

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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
