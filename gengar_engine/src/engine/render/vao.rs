// use crate::engine::render::
use crate::engine::vectors::*;

pub struct Vao {
    pub id: u32,
    buffers: Vec<u32>,
}

impl Vao {
    pub fn new_empty() -> Self {
        Vao {
            id: 0,
            buffers: vec![],
        }
    }

    pub fn new(render_api: &impl super::RenderApi) -> Self {
        let id = render_api.create_vao().unwrap();
        Vao {
            id: id,
            buffers: vec![],
        }
    }

    pub fn upload_v3(
        &mut self,
        render_api: &impl super::RenderApi,
        data: Vec<VecThreeFloat>,
        location: u32,
    ) {
        render_api.vao_upload_v3(self, data, location).unwrap();
    }

    pub fn add_buffer(&mut self, buffer: u32) {
        self.buffers.push(buffer);
    }
}
