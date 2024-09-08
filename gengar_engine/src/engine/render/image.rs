pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub gl_id: Option<u32>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            width: 0,
            height: 0,
            data: vec![],
            gl_id: None,
        }
    }
}
