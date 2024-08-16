use gengar_engine::engine::model::*;
use gengar_engine::engine::render::vao::*;

pub struct State {
    pub cube_model: Model,
    pub cube_vao: Vao,
}

impl State {
    pub fn new() -> Self {
        State {
            cube_model: Model::new(),
            cube_vao: Vao::new_empty(),
        }
    }
}
