use crate::engine::vectors::*;

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<VecThreeFloat>,
    pub indices: Vec<u32>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            vertices: vec![],
            indices: vec![],
        }
    }
}
