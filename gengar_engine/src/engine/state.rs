use crate::engine::render::camera::*;
use crate::engine::render::render_command::*;
use crate::engine::render::shader::*;
use crate::engine::render::vao::*;

pub struct State {
    pub basic_shader: Shader,
    pub cube: Vao,

    pub frame: i64,

    pub render_commands: Vec<RenderCommand>,

    pub camera: Camera,
}

impl State {
    pub fn new() -> Self {
        State {
            basic_shader: Shader::new_empty(),
            cube: Vao::new_empty(),
            frame: 0,
            render_commands: vec![],
            camera: Camera::new(),
        }
    }
}
