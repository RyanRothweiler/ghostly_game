pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new() -> Self {
        Image {
            width: 0,
            height: 0,
            data: vec![],
        }
    }
}
