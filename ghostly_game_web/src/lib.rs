#![allow(unused_variables, unused_imports)]

mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

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
    log("main loop");
}
