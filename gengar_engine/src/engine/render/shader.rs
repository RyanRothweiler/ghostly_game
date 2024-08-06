use crate::engine::error::*;
use crate::engine::render::RenderApi;

pub struct Shader {
    pub prog_id: u32,
}

impl Shader {
    pub fn new_empty() -> Self {
        Shader { prog_id: 0 }
    }

    pub fn compile(vert: &str, frag: &str, render_api: &impl RenderApi) -> Result<Self, Error> {
        let prog_id = render_api.make_shader_program(vert, frag)?;
        Ok(Shader { prog_id: prog_id })
    }
}
