use crate::vectors::*;

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<VecThreeFloat>,
    pub uvs: Vec<VecTwo>,
    pub indices: Vec<u32>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            vertices: vec![],
            uvs: vec![],
            indices: vec![],
        }
    }
}
