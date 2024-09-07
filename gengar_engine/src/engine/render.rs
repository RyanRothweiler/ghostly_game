use crate::engine::{error::*, state::*, vectors::*};
use png;
use std::{fs::File, path::Path};

pub mod camera;
pub mod image;
pub mod render_command;
pub mod shader;
pub mod vao;

use image::*;
use render_command::*;
use shader::*;

// Render backend independent calls
pub trait RenderApi {
    fn make_shader_program(&self, vert_shader: &str, frag_shader: &str) -> Result<u32, Error>;
    fn create_vao(&self) -> Result<u32, Error>;
    fn upload_texture(&self, image: &Image) -> Result<u32, Error>;

    fn vao_upload_v3(
        &self,
        vao: &mut vao::Vao,
        data: &Vec<VecThreeFloat>,
        indices: &Vec<u32>,
        location: u32,
    ) -> Result<(), Error>;

    fn vao_upload_v2(
        &self,
        vao: &mut vao::Vao,
        data: &Vec<VecTwo>,
        location: u32,
    ) -> Result<(), Error>;
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

pub fn load_image(path: &Path) -> Result<Image, Error> {
    let mut image = Image::new();

    let image_dec = png::Decoder::new(File::open(path)?);
    let mut reader = image_dec.read_info().unwrap();
    image.data = vec![0; reader.output_buffer_size()];

    let info = reader.next_frame(&mut image.data).unwrap();
    image.width = info.width;
    image.height = info.height;

    Ok(image)
}
