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

    // albedo
    {
        let image_bytes_cursor =
            Cursor::new(include_bytes!("../resources/monkey_testing/BaseColor.png"));
        gs.albedo = load_image(image_bytes_cursor).unwrap();
        gs.albedo.gl_id = Some(render_api.upload_texture(&gs.albedo, true).unwrap());
    }

    // metallic
    {
        let image_bytes_cursor =
            Cursor::new(include_bytes!("../resources/monkey_testing/Metallic.png"));
        gs.metallic = load_image(image_bytes_cursor).unwrap();
        gs.metallic.gl_id = Some(render_api.upload_texture(&gs.metallic, false).unwrap());
    }

    // normal
    {
        let image_bytes_cursor =
            Cursor::new(include_bytes!("../resources/monkey_testing/Normal.png"));
        gs.normal = load_image(image_bytes_cursor).unwrap();
        gs.normal.gl_id = Some(render_api.upload_texture(&gs.normal, false).unwrap());
    }

    // roughness
    {
        let image_bytes_cursor =
            Cursor::new(include_bytes!("../resources/monkey_testing/Roughness.png"));
        gs.roughness = load_image(image_bytes_cursor).unwrap();
        gs.roughness.gl_id = Some(render_api.upload_texture(&gs.roughness, false).unwrap());
    }

    // ao
    {
        let image_bytes_cursor = Cursor::new(include_bytes!("../resources/monkey_testing/AO.png"));
        gs.ao = load_image(image_bytes_cursor).unwrap();
        gs.ao.gl_id = Some(render_api.upload_texture(&gs.ao, true).unwrap());
    }

    // monkey material
    gs.monkey_material.shader = Some(es.basic_shader);
    gs.monkey_material.uniforms.insert(
        "tex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: gs.albedo.gl_id.unwrap(),
            texture_slot: 0,
        }),
    );
    gs.monkey_material.uniforms.insert(
        "normalTex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: gs.normal.gl_id.unwrap(),
            texture_slot: 1,
        }),
    );
    gs.monkey_material.uniforms.insert(
        "metallicTex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: gs.metallic.gl_id.unwrap(),
            texture_slot: 2,
        }),
    );
    gs.monkey_material.uniforms.insert(
        "roughnessTex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: gs.roughness.gl_id.unwrap(),
            texture_slot: 3,
        }),
    );
    gs.monkey_material.uniforms.insert(
        "aoTex".to_string(),
        UniformData::Texture(TextureInfo {
            image_id: gs.ao.gl_id.unwrap(),
            texture_slot: 4,
        }),
    );

    gs.center_trans = Some(es.new_transform());
    gs.monkey_trans = Some(es.new_transform());
    gs.light_trans = Some(es.new_transform());

    let mt: &mut Transform = &mut es.transforms[gs.monkey_trans.unwrap()];

    let ct: &mut Transform = &mut es.transforms[gs.center_trans.unwrap()];
    ct.local_rotation.y = 90.0;

    let lt: &mut Transform = &mut es.transforms[gs.light_trans.unwrap()];
    lt.local_position.x = 3.5;
    lt.local_position.y = 3.5;
    lt.parent = gs.center_trans;
}

#[no_mangle]
pub fn game_loop(gs: &mut State, es: &mut EngineState, input: &Input) {
    gengar_engine::debug::init_context(es.shader_color.clone(), es.model_sphere.clone());
    gengar_engine::debug::frame_start();

    // rotating monkey
    {
        if input.mouse_left.pressing {
            let sens = 0.001;
            gs.monkey_vel.y = gs.monkey_vel.y + (input.mouse_pos_delta.x * sens);
            gs.monkey_vel.x = gs.monkey_vel.x + (input.mouse_pos_delta.y * sens);
        }

        gs.monkey_vel = gs.monkey_vel * 0.9;

        let monkey_transform: &mut Transform = &mut es.transforms[gs.monkey_trans.unwrap()];
        monkey_transform.local_rotation.y = monkey_transform.local_rotation.y + gs.monkey_vel.y;
        monkey_transform.local_rotation.x = monkey_transform.local_rotation.x + gs.monkey_vel.x;
    }

    // draw sphere for light
    {
        let ct: &mut Transform = &mut es.transforms[gs.light_trans.unwrap()];
        draw_sphere(ct.global_matrix.get_position(), 0.1, Color::white());
    }

    es.render_commands.push(RenderCommand::new_model(
        &es.transforms[gs.monkey_trans.unwrap()],
        &gs.model_monkey,
        &gs.monkey_material,
    ));

    es.game_debug_render_commands = gengar_engine::debug::get_render_list().clone();
}
