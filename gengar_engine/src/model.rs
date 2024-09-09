use crate::{render::vao::Vao, vectors::*};

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<VecThreeFloat>,
    pub uvs: Vec<VecTwo>,
    pub indices: Vec<u32>,
    pub vao: Option<Vao>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            vertices: vec![],
            uvs: vec![],
            indices: vec![],
            vao: None,
        }
    }
}
