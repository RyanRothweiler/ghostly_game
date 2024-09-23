use crate::{
    color::*,
    matricies::matrix_four_four::*,
    model::*,
    render::{camera::*, material::*, render_command::*, shader::*},
    state::*,
    transform::*,
    vectors::*,
};

use std::cell::RefCell;

pub struct DebugContext {
    pub render_commands: Vec<RenderCommand>,
    pub shader_color: Shader,
    pub model_sphere: Model,
}

static mut DEBUG_CONTEXT: Option<DebugContext> = None;

pub fn init_context(shader_color: Shader, model_sphere: Model) {
    unsafe {
        DEBUG_CONTEXT = Some(DebugContext {
            render_commands: vec![],
            shader_color,
            model_sphere,
        });
    }
}

pub fn frame_start() {
    unsafe {
        DEBUG_CONTEXT.as_mut().as_mut().unwrap().render_commands = vec![];
    }
}

pub fn get_render_list<'a>() -> &'a mut Vec<RenderCommand> {
    unsafe {
        return &mut DEBUG_CONTEXT.as_mut().unwrap().render_commands;
    }
}

pub fn draw_sphere(center: VecThreeFloat, size: f64, color: Color) {
    unsafe {
        let context: &mut DebugContext = DEBUG_CONTEXT.as_mut().unwrap();

        let mut trans = Transform::new();
        trans.local_position = center;
        trans.local_scale = VecThreeFloat::new(size, size, size);
        trans.update_global_matrix(&M44::new_identity());

        let mut material = Material::new();
        material.shader = Some(context.shader_color);
        material.uniforms.insert(
            "color".to_string(),
            UniformData::VecFour(VecFour::from(color)),
        );

        let model_sphere = context.model_sphere.clone();

        context
            .render_commands
            .push(RenderCommand::new_model(&trans, &model_sphere, &material));
    }
}
