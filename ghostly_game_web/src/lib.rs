#![allow(unused_variables, unused_imports, dead_code, unused_assignments)]

use gengar_engine::engine::{state::State as EngineState, vectors::*};
// use gengar_render_opengl::ogl_render::*;

use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod render_api;
mod utils;
mod webgl;

use render_api::*;
use webgl::{webgl_render::*, webgl_render_api::*};

static mut MAIN_FIRST: bool = true;

static mut ENGINE_STATE: Option<EngineState> = None;
static mut RENDER_API: Option<WebGLRenderApi> = None;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn log(input: &str) {
    console::log_1(&input.into());
}

#[wasm_bindgen(start)]
pub fn start() {}

#[wasm_bindgen]
pub fn main_loop() {
    unsafe {
        // TODO get the actual window resolution
        let resolution = VecTwo::new(512.0, 512.0);

        // First loop init stuff
        if MAIN_FIRST {
            MAIN_FIRST = false;

            let document = web_sys::window().unwrap().document().unwrap();

            let canvas = document.get_element_by_id("gengar_canvas").unwrap();

            let canvas: web_sys::HtmlCanvasElement =
                canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

            let gl_context = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .unwrap();

            // set webgl global state
            let gl_state = webgl::webgl_render_api::WebGLState {
                programs: HashMap::new(),
                next_prog_id: 0,
            };
            webgl::webgl_render_api::GL_STATE = Some(gl_state);
            webgl::webgl_render_api::GL_CONTEXT = Some(gl_context);

            RENDER_API = Some(get_render_api());
            ENGINE_STATE = Some(gengar_engine::engine::state::State::new(resolution));

            gengar_engine::engine::load_resources(
                &mut ENGINE_STATE.as_mut().unwrap(),
                RENDER_API.as_mut().unwrap(),
            );

            // (game_dll.proc_init)(&mut game_state, &render_api);
        }

        // engine::engine_frame_start(&mut engine_state, &input, &render_api);
        // (game_dll.proc_loop)(&mut game_state, &mut engine_state, &input);
        // engine::engine_frame_end(&mut engine_state);

        render(
            &ENGINE_STATE.as_mut().unwrap(),
            &RENDER_API.as_mut().unwrap(),
        );
    }
}
