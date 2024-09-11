use crate::{
    matricies::matrix_four_four::*,
    model::*,
    render::{camera::*, material::Material, shader::*, vao::Vao},
    transform::*,
    vectors::*,
};
use std::collections::HashMap;

pub struct RenderCommand {
    pub vao_id: u32,
    pub prog_id: u32,
    pub indices: Vec<u32>,
    pub uniforms: HashMap<String, UniformData>,
}

impl RenderCommand {
    pub fn new_model(
        transform: &Transform,
        model: &Model,
        material: &Material,
        cam: &Camera,
    ) -> Self {
        let mut uniforms: HashMap<String, UniformData> = material.uniforms.clone();

        let mut mat = M44::new_identity();
        mat.translate(transform.position);
        // mat.rotate_y(offset);
        // mat.rotate_x(offset);
        // mat.rotate_z(offset);

        uniforms.insert("model".to_string(), UniformData::M44(mat.clone()));

        // todo move these into the render step
        uniforms.insert("view".to_string(), UniformData::M44(cam.view_mat));
        uniforms.insert(
            "projection".to_string(),
            UniformData::M44(cam.projection_mat),
        );

        RenderCommand {
            vao_id: model.vao.id,
            prog_id: material.shader.unwrap().prog_id,
            indices: model.indices.clone(),
            uniforms: uniforms,
        }
    }
}
