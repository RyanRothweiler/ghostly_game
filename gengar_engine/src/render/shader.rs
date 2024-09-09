use crate::{error::*, matricies::matrix_four_four::*, render::RenderApi};
use std::collections::HashMap;

pub struct Shader {
    pub prog_id: u32,
    pub uniforms: HashMap<String, UniformData>,
}

#[derive(Clone)]
pub enum UniformData {
    M44(M44),
    Texture(u32),
}

impl Shader {
    pub fn new_empty() -> Self {
        Shader {
            prog_id: 0,
            uniforms: HashMap::new(),
        }
    }

    pub fn compile(vert: &str, frag: &str, render_api: &impl RenderApi) -> Result<Self, Error> {
        let prog_id = render_api.make_shader_program(vert, frag)?;
        Ok(Shader {
            prog_id: prog_id,
            uniforms: HashMap::new(),
        })
    }

    pub fn set_uniform(&mut self, name: &str, data: UniformData) {
        self.uniforms.insert(name.to_string(), data);
    }
}
