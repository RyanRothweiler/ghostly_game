use gengar_engine::{
    model::*,
    render::{image::Image, vao::*},
};

pub struct State {
    pub cube_model: Model,
    pub cube_vao: Vao,
    pub texture: Image,
}

impl State {
    pub fn new() -> Self {
        State {
            cube_model: Model::new(),
            cube_vao: Vao::new_empty(),
            texture: Image::new(),
        }
    }
}
