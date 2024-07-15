#![allow(unused_imports, unused_variables)]

use gengar_engine::engine::color::Color;
use gengar_engine::engine::*;

pub struct RenderApi {
    pub clear: fn(color: Color),
}

pub fn setup(platform_api: &PlatformApi) {}

pub fn render(render_api: &RenderApi) {}
