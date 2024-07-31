pub struct State {
    pub prog_id: u32,
    pub cube_id: u32,
}

impl State {
    pub fn new() -> Self {
        State {
            prog_id: 0,
            cube_id: 0,
        }
    }
}
