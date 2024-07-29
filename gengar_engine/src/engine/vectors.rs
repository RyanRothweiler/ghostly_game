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
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<VecThreeFloat> for VecThreeFloatC {
    fn from(input: VecThreeFloat) -> Self {
        VecThreeFloatC {
            x: input.x,
            y: input.y,
            z: input.z,
        }
    }
}
