use crate::{error::*, matricies::matrix_four_four::*, render::RenderApi, vectors::*};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Shader {
    pub prog_id: u32,
}

#[derive(Clone)]
pub enum UniformData {
    M44(M44),
    Texture(u32),
    VecFour(VecFour),
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
