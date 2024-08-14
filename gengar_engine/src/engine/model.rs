use crate::engine::vectors::*;

pub struct Model {
    pub vertices: Vec<VecThreeFloat>,
}

impl Model {
    pub fn new() -> Self {
        Model { vertices: vec![] }
    }
}
