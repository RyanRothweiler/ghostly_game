use crate::engine::error::*;

// Render backend independent calls
pub trait RenderApi {
    fn make_shader_program(&self, vert_shader: &str, frag_shader: &str) -> Result<i32, Error>;
}

pub enum ShaderType {
    Vertex,
    Fragment,
}
