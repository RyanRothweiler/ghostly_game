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

    // brick texture
    {
        let image_bytes_cursor = Cursor::new(include_bytes!("../resources/brick.png"));
        gs.texture = load_image(image_bytes_cursor).unwrap();
        gs.texture.gl_id = Some(render_api.upload_texture(&gs.texture, true).unwrap());
    }

    // normal map
    {
        let image_bytes_cursor = Cursor::new(include_bytes!("../resources/normal_map.png"));
        gs.texture_normal = load_image(image_bytes_cursor).unwrap();
        gs.texture_normal.gl_id = Some(
            render_api
                .upload_texture(&gs.texture_normal, false)
                .unwrap(),
        );
    }

    // monkey material
    gs.monkey_material.shader = Some(es.basic_shader);
    gs.monkey_material.uniforms.insert(
        "tex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: gs.texture.gl_id.unwrap(),
            texture_slot: 0,
        }),
    );
    gs.monkey_material.uniforms.insert(
        "normalTex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: gs.texture_normal.gl_id.unwrap(),
            texture_slot: 1,
        }),
    );

    /*
    gs.monkey_material.uniforms.insert(
        "color".to_string(),
        UniformData::VecFour(VecFour::new(0.0, 1.0, 0.0, 1.0)),
    );
    */

    gs.center_trans = Some(es.new_transform());
    gs.monkey_trans = Some(es.new_transform());
    gs.light_trans = Some(es.new_transform());

    let mt: &mut Transform = &mut es.transforms[gs.monkey_trans.unwrap()];
    // mt.parent = gs.center_trans;

    let ct: &mut Transform = &mut es.transforms[gs.center_trans.unwrap()];
    // ct.local_position.y = 1.5;

    let lt: &mut Transform = &mut es.transforms[gs.light_trans.unwrap()];
    lt.local_position.x = 5.0;
    lt.parent = gs.center_trans;
}

#[no_mangle]
pub fn game_loop(gs: &mut State, es: &mut EngineState, input: &Input) {
    gengar_engine::debug::init_context(es.shader_color.clone(), es.model_sphere.clone());
    gengar_engine::debug::frame_start();

    es.camera.move_fly_(0.05, input);

    // gs.monkey_second_transform.rotation.x = gs.monkey_second_transform.rotation.x + 0.01;
    // gs.monkey_second_transform.rotation.y = gs.monkey_second_transform.rotation.y + 0.01;
    // gs.monkey_second_transform.rotation.z = gs.monkey_second_transform.rotation.z + 0.01;

    // gs.monkey_second_transform.position.z = 10.0;

    let right_trans = Transform::new();

    let left_trans = Transform::new();

    let mut y_trans = Transform::new();
    y_trans.local_position.y = 1.5;

    {
        let mt: &mut Transform = &mut es.transforms[gs.monkey_trans.unwrap()];
        // mt.local_position.y = 1.5;
        // mt.local_rotation.x = mt.local_rotation.x + 0.01;
        // mt.local_rotation.y = mt.local_rotation.y + 0.01;
        // mt.local_rotation.z = mt.local_rotation.z + 0.01;
    }

    {
        let ct: &mut Transform = &mut es.transforms[gs.center_trans.unwrap()];
        // mt.local_position.y = 1.5;
        // ct.local_rotation.z = ct.local_rotation.z + 0.01;
    }

    {
        let ct: &mut Transform = &mut es.transforms[gs.light_trans.unwrap()];
        // mt.local_position.y = 1.5;
        ct.local_position.x = -5.0;

        draw_sphere(ct.global_matrix.get_position(), 0.1, Color::blue());
    }

    es.render_commands.push(RenderCommand::new_model(
        &es.transforms[gs.monkey_trans.unwrap()],
        &gs.model_monkey,
        &gs.monkey_material,
    ));

    /*
    es.render_commands.push(RenderCommand::new_model(
        &left_trans,
        &gs.model_monkey,
        &gs.monkey_material,
    ));

    es.render_commands.push(RenderCommand::new_model(
        &y_trans,
        &gs.model_monkey,
        &gs.monkey_material,
    ));

    es.render_commands.push(RenderCommand::new_model(
        &gs.monkey_second_transform,
        &gs.model_monkey,
        &gs.monkey_material,
    ));
    */

    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
