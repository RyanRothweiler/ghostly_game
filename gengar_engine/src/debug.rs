use crate::{
    color::*,
    model::*,
    render::{material::*, render_command::*, shader::*},
    state::*,
    transform::*,
    vectors::*,
};

pub fn draw_sphere(center: VecThreeFloat, _size: f64, color: Color, es: &mut State) {
    let mut trans = Transform::new();
    trans.position = center;

    let mut material = Material::new();
    material.shader = Some(es.shader_color);
    material.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(VecFour::from(color)),
    );

    es.render_commands.push(RenderCommand::new_model(
        &trans,
        &es.model_sphere,
        &material,
        &es.camera,
    ));
}
