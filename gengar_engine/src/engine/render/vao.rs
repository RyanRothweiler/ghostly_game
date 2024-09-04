// use crate::engine::render::
use crate::engine::vectors::*;

pub struct Vao {
    pub id: u32,
    pub index_buffer: u32,
}

impl Vao {
    pub fn new_empty() -> Self {
        Vao {
            id: 0,
            index_buffer: 0,
        }
    }

    pub fn new(render_api: &impl super::RenderApi) -> Self {
        let mut ret = Self::new_empty();
        ret.id = render_api.create_vao().unwrap();
        ret
    }

    pub fn upload_v3(
        &mut self,
        render_api: &impl super::RenderApi,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) {
        render_api
            .vao_upload_v3(self, data, indices, location)
            .unwrap();
    }
}
