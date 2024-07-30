use crate::engine::error::*;
use crate::engine::vectors::*;

pub mod vao;

// Render backend independent calls
pub trait RenderApi {
    fn make_shader_program(&self, vert_shader: &str, frag_shader: &str) -> Result<i32, Error>;

    fn create_vao(&self) -> Result<u32, Error>;

    fn vao_upload_v3(
        &self,
        vao: &mut vao::Vao,
        data: Vec<VecThreeFloat>,
        location: u32,
    ) -> Result<(), Error>;

    fn render(&self, prog_id: u32);
}

pub enum ShaderType {
    Vertex,
    Fragment,
}
