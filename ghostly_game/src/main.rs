#![allow(while_true)]

use gengar_engine::engine;

fn main() {
    engine::engine_loop(game_loop);
}

fn game_loop() {
    println!("do game stuff here");
}
