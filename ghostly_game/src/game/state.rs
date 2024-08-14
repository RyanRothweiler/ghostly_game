use gengar_engine::engine::model::*;

pub struct State {
    pub cube_model: Model,
}

impl State {
    pub fn new() -> Self {
        State {
            cube_model: Model::new(),
        }
    }
}
