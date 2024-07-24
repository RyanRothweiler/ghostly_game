// Render backend independent calls
pub trait RenderApi {
    fn make_shader_program(&self, vert_shader: &str, frag_shader: &str) -> i32;
}

pub enum ShaderType {
    Vertex,
    Fragment,
}
