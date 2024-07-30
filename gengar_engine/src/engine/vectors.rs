pub struct VecThreeFloat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VecThreeFloat {
    pub fn new(x: f64, y: f64, z: f64) -> VecThreeFloat {
        VecThreeFloat { x, y, z }
    }
}

// Used for any FFI C stuff. Like ogl graphics rendering
#[repr(C)]
pub struct VecThreeFloatC {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<VecThreeFloat> for VecThreeFloatC {
    fn from(input: VecThreeFloat) -> Self {
        VecThreeFloatC {
            x: input.x as f32,
            y: input.y as f32,
            z: input.z as f32,
        }
    }
}
