use gengar_engine::{
    model::*,
    render::{image::Image, material::*, vao::*},
    transform::*,
    vectors::*,
};

pub struct State {
    pub model_monkey: Model,

    pub albedo: Image,
    pub metallic: Image,
    pub normal: Image,
    pub roughness: Image,
    pub ao: Image,

    pub monkey_material: Material,
    pub monkey_trans: Option<usize>,
    pub center_trans: Option<usize>,

    pub light_trans: Option<usize>,

    pub monkey_vel: VecTwo,
}

impl State {
    pub fn new() -> Self {
        State {
            model_monkey: Model::new(),

            albedo: Image::new(),
            metallic: Image::new(),
            normal: Image::new(),
            roughness: Image::new(),
            ao: Image::new(),

            monkey_material: Material::new(),

            monkey_vel: VecTwo::new(0.0, 0.0),

            monkey_trans: None,
            center_trans: None,
            light_trans: None,
        }
    }
}
