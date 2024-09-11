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
    gs.model_monkey =
        Model::load_upload(include_str!("../resources/monkey.obj"), render_api).unwrap();

    // load image
    let image_bytes = include_bytes!("../resources/brick.png");
    let image_bytes_cursor = Cursor::new(image_bytes);

    gs.texture = load_image(image_bytes_cursor).unwrap();
    gs.texture.gl_id = Some(render_api.upload_texture(&gs.texture).unwrap());

    // monkey material
    gs.monkey_material.shader = Some(es.shader_color);
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
    // camera controls
    {
        let cam_speed = 0.05;
        if input.keyboard[ASCII_A].pressing {
            es.camera.transform.position.x = es.camera.transform.position.x - cam_speed;
        }
        if input.keyboard[ASCII_D].pressing {
            es.camera.transform.position.x = es.camera.transform.position.x + cam_speed;
        }
        if input.keyboard[ASCII_S].pressing {
            es.camera.transform.position.y = es.camera.transform.position.y - cam_speed;
        }
        if input.keyboard[ASCII_W].pressing {
            es.camera.transform.position.y = es.camera.transform.position.y + cam_speed;
        }
        if input.keyboard[ASCII_Q].pressing {
            es.camera.transform.position.z = es.camera.transform.position.z + cam_speed;
        }
        if input.keyboard[ASCII_E].pressing {
            es.camera.transform.position.z = es.camera.transform.position.z - cam_speed;
        }
        es.camera.update_matricies();
    }

    gs.monkey_second_transform.position.x = 5.0;

    let offset: f64 = (es.frame as f64) * 0.01;

    /*
    es.render_commands.push(RenderCommand::new_model(
        &gs.monkey_transform,
        &gs.model_monkey,
        &gs.monkey_material,
        &es.camera,
    ));

    es.render_commands.push(RenderCommand::new_model(
        &gs.monkey_second_transform,
        &gs.model_monkey,
        &gs.monkey_material,
        &es.camera,
    ));
    */

    draw_sphere(
        VecThreeFloat::new(0.0, 1.0, 0.0),
        1.0,
        Color::new(0.0, 1.0, 0.0, 1.0),
        es,
    );

    draw_sphere(
        VecThreeFloat::new(5.0, 1.0, 0.0),
        1.0,
        Color::new(0.0, 0.0, 1.0, 1.0),
        es,
    );
}
