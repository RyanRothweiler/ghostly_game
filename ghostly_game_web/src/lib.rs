#![allow(unused_variables, unused_imports)]

mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

use std::cell::RefCell;
use std::rc::Rc;

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

    // gl_context.canvas().unwrap().width = 100;

    gl_context.viewport(0, 0, 1000, 1000);
    gl_context.clear_color(1.0, 0.0, 0.0, 1.0);
    gl_context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    log("main loop");
}
