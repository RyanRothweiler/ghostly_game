#![allow(unused_imports, unused_variables)]

pub mod state;

use crate::state::*;
use gengar_engine::{
    ascii::*,
    color::*,
    debug::*,
    matricies::matrix_four_four::*,
    model::*,
    obj,
    render::{
        image::Image, load_image, material::*, render_command::RenderCommand, shader::*, vao::*,
        RenderApi,
    },
    state::Input,
    state::State as EngineState,
    transform::*,
    vectors::*,
};
use gengar_render_opengl::*;
use std::{fs::File, io::Cursor, path::Path};

// The render_api is hard-coded here instead of using a trait so that we can support hot reloading
#[no_mangle]
pub fn game_init_ogl(gs: &mut State, es: &mut EngineState, render_api: &OglRenderApi) {
    game_init(gs, es, render_api);
}

pub fn game_init(gs: &mut State, es: &mut EngineState, render_api: &impl RenderApi) {
    gengar_engine::debug::init_context(es.shader_color.clone(), es.model_sphere.clone());

    gs.model_monkey =
        Model::load_upload(include_str!("../resources/monkey.obj"), render_api).unwrap();

    // load image
    let image_bytes = include_bytes!("../resources/brick.png");
    let image_bytes_cursor = Cursor::new(image_bytes);

    gs.texture = load_image(image_bytes_cursor).unwrap();
    gs.texture.gl_id = Some(render_api.upload_texture(&gs.texture).unwrap());

    // monkey material
    gs.monkey_material.shader = Some(es.basic_shader);
    gs.monkey_material.uniforms.insert(
        "texture0".to_string(),
        UniformData::Texture(gs.texture.gl_id.unwrap()),
    );
    gs.monkey_material.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(VecFour::new(0.0, 1.0, 0.0, 1.0)),
    );
}

#[no_mangle]
pub fn game_loop(gs: &mut State, es: &mut EngineState, input: &Input) {
    gengar_engine::debug::init_context(es.shader_color.clone(), es.model_sphere.clone());
    gengar_engine::debug::frame_start();

    draw_sphere(
        VecThreeFloat::new(5.0, 0.0, 0.0),
        0.1,
        Color::new(0.0, 0.0, 1.0, 1.0),
    );

    es.camera.move_fly(0.05, input);

    // gs.monkey_second_transform.rotation.x = gs.monkey_second_transform.rotation.x + 0.01;
    // gs.monkey_second_transform.rotation.y = gs.monkey_second_transform.rotation.y + 0.01;
    // gs.monkey_second_transform.rotation.z = gs.monkey_second_transform.rotation.z + 0.01;

    es.render_commands.push(RenderCommand::new_model(
        &gs.monkey_second_transform,
        &gs.model_monkey,
        &gs.monkey_material,
    ));

    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
