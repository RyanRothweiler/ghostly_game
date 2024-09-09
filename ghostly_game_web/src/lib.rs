#![allow(unused_variables, unused_imports, dead_code, unused_assignments)]

use gengar_engine::engine::{state::Input, state::State as EngineState, vectors::*};
use ghostly_game::game::{game_init, game_loop, state::*};

use wasm_bindgen::prelude::*;
use web_sys::{console, KeyboardEvent, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

mod utils;
mod webgl;

use webgl::{webgl_render::*, webgl_render_api::*};

static mut ENGINE_STATE: Option<EngineState> = None;
static mut GAME_STATE: Option<ghostly_game::game::state::State> = None;
static mut RENDER_API: Option<WebGLRenderApi> = None;
static mut INPUT: Option<Input> = None;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub fn log(input: &str) {
    console::log_1(&input.into());
}

#[wasm_bindgen(start)]
pub fn start() {
    let gl_state = webgl::webgl_render_api::WebGLState {
        programs: HashMap::new(),
        next_prog_id: 0,

        vaos: HashMap::new(),
        next_vao_id: 0,

        textures: HashMap::new(),
        next_texture_id: 0,
    };

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("gengar_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let resolution = VecTwo::new(canvas.client_width() as f64, canvas.client_height() as f64);

    let gl_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    unsafe {
        webgl::webgl_render_api::GL_STATE = Some(gl_state);
        webgl::webgl_render_api::GL_CONTEXT = Some(gl_context);

        RENDER_API = Some(get_render_api());
        INPUT = Some(Input::new());
        GAME_STATE = Some(ghostly_game::game::state::State::new());
        ENGINE_STATE = Some(gengar_engine::engine::state::State::new(resolution));

        gengar_engine::engine::load_resources(
            &mut ENGINE_STATE.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
        );

        game_init(GAME_STATE.as_mut().unwrap(), RENDER_API.as_mut().unwrap());
    };
}

#[wasm_bindgen]
pub fn key_down(vent: KeyboardEvent) {
    let input: &mut Input = unsafe { INPUT.as_mut().unwrap() };
    input.keyboard[vent.key_code() as usize].update(true);
}

#[wasm_bindgen]
pub fn key_up(vent: KeyboardEvent) {
    let input: &mut Input = unsafe { INPUT.as_mut().unwrap() };
    input.keyboard[vent.key_code() as usize].update(false);
}

#[wasm_bindgen]
pub fn main_loop() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("gengar_canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let resolution = VecTwo::new(canvas.client_width() as f64, canvas.client_height() as f64);

    let gl_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    unsafe {
        gengar_engine::engine::engine_frame_start(
            ENGINE_STATE.as_mut().unwrap(),
            INPUT.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
        );
        game_loop(
            GAME_STATE.as_mut().unwrap(),
            ENGINE_STATE.as_mut().unwrap(),
            INPUT.as_mut().unwrap(),
        );
        gengar_engine::engine::engine_frame_end(ENGINE_STATE.as_mut().unwrap());

        render(
            ENGINE_STATE.as_mut().unwrap(),
            RENDER_API.as_mut().unwrap(),
            &resolution,
            &gl_context,
        );
    }
}
