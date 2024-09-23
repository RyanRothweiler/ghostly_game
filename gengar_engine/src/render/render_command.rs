use crate::{
    matricies::matrix_four_four::*,
    model::*,
    render::{camera::*, material::Material, shader::*, vao::Vao},
    transform::*,
    vectors::*,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RenderCommand {
    pub vao_id: u32,
    pub prog_id: u32,
    pub indices: Vec<u32>,
    pub uniforms: HashMap<String, UniformData>,
}

impl RenderCommand {
    pub fn new_model(transform: &Transform, model: &Model, material: &Material) -> Self {
        let mut uniforms: HashMap<String, UniformData> = material.uniforms.clone();

        uniforms.insert(
            "model".to_string(),
            UniformData::M44(transform.global_matrix.clone()),
        );

        uniforms.insert(
            "lightPos".to_string(),
            UniformData::M44(transform.global_matrix.clone()),
        );

        RenderCommand {
            vao_id: model.vao.id,
            prog_id: material.shader.unwrap().prog_id,
            indices: model.indices.clone(),
            uniforms: uniforms,
        }
    }
}
