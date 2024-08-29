use crate::engine::error::*;
use crate::engine::state::*;
use crate::engine::vectors::*;

pub mod camera;
pub mod render_command;
pub mod shader;
pub mod vao;

use render_command::*;
use shader::*;

// Render backend independent calls
pub trait RenderApi {
    fn make_shader_program(&self, vert_shader: &str, frag_shader: &str) -> Result<u32, Error>;

    fn create_vao(&self) -> Result<u32, Error>;

    fn vao_upload_v3(
        &self,
        vao: &mut vao::Vao,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) -> Result<(), Error>;
}

pub enum ShaderType {
    Vertex,
    Fragment,
}
