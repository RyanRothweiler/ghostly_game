use std::collections::HashMap;

use crate::engine::render::camera::*;
use crate::engine::render::shader::*;
use crate::engine::render::vao::Vao;

pub struct RenderCommand {
    pub vao_id: u32,
    pub prog_id: u32,
    pub indecies: Vec<u32>,
    pub uniforms: HashMap<String, UniformData>,
}

impl RenderCommand {
    pub fn new_model(vao: &Vao, shader: &Shader, indecies: Vec<u32>, cam: &Camera) -> Self {
        let mut uniforms: HashMap<String, UniformData> = shader.uniforms.clone();

        uniforms.insert("view".to_string(), UniformData::M44(cam.view_mat));

        RenderCommand {
            vao_id: vao.id,
            prog_id: shader.prog_id,
            indecies: indecies,
            uniforms: uniforms,
        }
    }
}
