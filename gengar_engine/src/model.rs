use crate::{
    error::Error,
    obj,
    render::{vao::Vao, RenderApi},
    vectors::*,
};

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<VecThreeFloat>,
    pub normals: Vec<VecThreeFloat>,
    pub uvs: Vec<VecTwo>,
    pub indices: Vec<u32>,
    pub vao: Vao,
}

impl Model {
    pub fn new() -> Self {
        Model {
            vertices: vec![],
            normals: vec![],
            uvs: vec![],
            indices: vec![],
            vao: Vao::new_empty(),
        }
    }

    // This assumes locations for shader layout data.
    // If the layout locations in the shader changes this will break
    pub fn load_upload(data: &str, render_api: &impl RenderApi) -> Result<Self, Error> {
        let mut model = obj::load(data)?;

        model.vao = Vao::new(render_api);

        // uplaod vertices
        model
            .vao
            .upload_v3(render_api, &model.vertices, &model.indices, 0)
            .unwrap();

        // upload uvs
        model.vao.upload_v2(render_api, &model.uvs, 1).unwrap();

        // uplaod normals
        model
            .vao
            .upload_v3(render_api, &model.normals, &model.indices, 2)
            .unwrap();

        Ok(model)
    }
}
