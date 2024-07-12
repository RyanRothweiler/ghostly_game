#[allow(unused_variables)]

pub fn engine_loop(game_loop: fn()) {
	loop {
		println!("engine loops");
		game_loop();
	}
}