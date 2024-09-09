use gengar_engine::{
    model::*,
    render::{image::Image, vao::*},
};

pub struct State {
    pub model_monkey: Model,
    pub texture: Image,
}

impl State {
    pub fn new() -> Self {
        State {
            model_monkey: Model::new(),
            texture: Image::new(),
        }
    }
}
