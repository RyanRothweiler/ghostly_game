use crate::engine::render::shader::Shader;
use crate::engine::render::vao::Vao;

pub struct RenderCommand {
    pub vao_id: u32,
    pub prog_id: u32,
    pub indecies: Vec<u32>,
}

impl RenderCommand {
    pub fn new_model(vao: &Vao, shader: &Shader, indecies: Vec<u32>) -> Self {
        RenderCommand {
            vao_id: vao.id,
            prog_id: shader.prog_id,
            indecies: indecies,
        }
    }
}
