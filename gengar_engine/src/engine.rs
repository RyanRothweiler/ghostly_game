#![allow(unused_variables, unused_imports)]

use std::thread;
use std::time::{Duration, SystemTime};

const FRAME_TARGET_FPS: f64 = 60.0;
const FRAME_TARGET: Duration = Duration::from_secs((1.0 / FRAME_TARGET_FPS) as u64);

pub fn engine_loop() {
    let time_start: SystemTime = SystemTime::now();

    println!("engine loop. doing engine stuff.");

    let time_end: SystemTime = SystemTime::now();
    let frame_duration: Duration = time_end.duration_since(time_start).unwrap();

    if FRAME_TARGET > frame_duration {
        let to_sleep: Duration = FRAME_TARGET - frame_duration;
        let slp = to_sleep.as_millis();
        thread::sleep(to_sleep);
    }
}
