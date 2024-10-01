use crate::{
    error::Error,
    obj,
    render::{vao::Vao, RenderApi},
    vectors::*,
};

#[derive(Debug, Clone)]
pub struct Model {
    pub vertices: Vec<VecThreeFloat>,
    pub uvs: Vec<VecTwo>,
    pub indices: Vec<u32>,
    pub vao: Vao,

    pub normals: Vec<VecThreeFloat>,
    pub normal_tans: Vec<VecThreeFloat>,
    pub normal_bi_tans: Vec<VecThreeFloat>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            vertices: vec![],
            uvs: vec![],
            indices: vec![],
            vao: Vao::new_empty(),

            normals: vec![],
            normal_tans: vec![],
            normal_bi_tans: vec![],
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

    // Highly unsafe. Assumes vertices in triangles and data linear / un-spooled / not indexed
    pub fn calculate_tangents(&mut self) {
        // clear old data
        self.normal_tans.clear();
        self.normal_bi_tans.clear();

        for i in (0..self.vertices.len()).step_by(3) {
            let point_one = self.vertices[i];
            let point_two = self.vertices[i + 1];
            let point_three = self.vertices[i + 2];

            let uv_one = self.uvs[i];
            let uv_two = self.uvs[i + 1];
            let uv_three = self.uvs[i + 2];

            let edge_one = point_one - point_two;
            let edge_two = point_three - point_two;

            let uv_delta_one = uv_two - uv_one;
            let uv_delta_two = uv_three - uv_one;

            let f = 1.0 / ((uv_delta_one.x * uv_delta_two.y) - (uv_delta_one.y * uv_delta_two.x));

            let mut tan = VecThreeFloat::new_zero();
            let mut bi_tan = VecThreeFloat::new_zero();

            tan.x = f * ((uv_delta_two.y * edge_one.x) - (uv_delta_one.y * edge_two.x));
            tan.y = f * ((uv_delta_one.y * edge_one.y) - (uv_delta_one.y * edge_two.y));
            tan.z = f * ((uv_delta_one.y * edge_one.z) - (uv_delta_one.y * edge_two.z));

            bi_tan.x = f * ((uv_delta_one.x * edge_two.x) - (uv_delta_one.x * edge_one.x));
            bi_tan.y = f * ((uv_delta_one.x * edge_two.y) - (uv_delta_one.x * edge_one.y));
            bi_tan.z = f * ((uv_delta_one.x * edge_two.z) - (uv_delta_one.x * edge_one.z));

            self.normal_tans.push(tan);
            self.normal_tans.push(tan);
            self.normal_tans.push(tan);

            self.normal_bi_tans.push(bi_tan);
            self.normal_bi_tans.push(bi_tan);
            self.normal_bi_tans.push(bi_tan);
        }
    }
}
