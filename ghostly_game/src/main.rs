#![allow(while_true)]

use gengar_engine::engine;
use gengar_render_webgl::render;

fn main() {
    loop {
        engine::engine_loop();
        game_loop();
        render::render();
    }
}

fn game_loop() {
    println!("do game stuff here");
}
