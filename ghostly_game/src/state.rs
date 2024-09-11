use gengar_engine::{
    model::*,
    render::{image::Image, material::*, vao::*},
    transform::*,
};

pub struct State {
    pub model_monkey: Model,
    pub texture: Image,
    pub monkey_transform: Transform,
    pub monkey_second_transform: Transform,
    pub monkey_material: Material,
}

impl State {
    pub fn new() -> Self {
        State {
            model_monkey: Model::new(),
            texture: Image::new(),
            monkey_transform: Transform::new(),
            monkey_second_transform: Transform::new(),
            monkey_material: Material::new(),
        }
    }
}
