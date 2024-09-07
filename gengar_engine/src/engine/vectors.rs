#[derive(Clone, Copy, Debug)]
pub struct VecThreeFloat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VecThreeFloat {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VecThreeFloat { x, y, z }
    }

    pub fn new_zero() -> Self {
        VecThreeFloat::new(0.0, 0.0, 0.0)
    }
}

// Used for any FFI C stuff. Like ogl graphics rendering
#[repr(C)]
pub struct VecThreeFloatC {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&VecThreeFloat> for VecThreeFloatC {
    fn from(input: &VecThreeFloat) -> Self {
        VecThreeFloatC {
            x: input.x as f32,
            y: input.y as f32,
            z: input.z as f32,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VecTwo {
    pub x: f64,
    pub y: f64,
}

impl VecTwo {
    pub fn new(x: f64, y: f64) -> Self {
        VecTwo { x, y }
    }
}

// Used for any FFI C stuff. Like ogl graphics rendering
#[repr(C)]
pub struct VecTwoC {
    pub x: f32,
    pub y: f32,
}

impl From<&VecTwo> for VecTwoC {
    fn from(input: &VecTwo) -> Self {
        Self {
            x: input.x as f32,
            y: input.y as f32,
        }
    }
}
