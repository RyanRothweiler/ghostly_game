use gengar_engine::{
    model::*,
    render::{image::Image, material::*, vao::*},
    transform::*,
};

pub struct State {
    pub model_monkey: Model,
    pub texture: Image,

    pub monkey_material: Material,
    pub monkey_trans: Option<usize>,
    pub center_trans: Option<usize>,

    pub light_trans: Option<usize>,
}

impl State {
    pub fn new() -> Self {
        State {
            model_monkey: Model::new(),
            texture: Image::new(),
            monkey_material: Material::new(),

            monkey_trans: None,
            center_trans: None,
            light_trans: None,
        }
    }
}
