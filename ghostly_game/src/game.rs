#![allow(unused_imports)]

use std::path::Path;

use gengar_engine::engine::*;

pub fn game_init() {
    obj::load(Path::new("testinghere")).unwrap();
}

pub fn game_loop() {}
