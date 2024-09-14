use crate::color::*;
use std::ops::{Add, Mul};

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

impl Add for VecThreeFloat {
    type Output = Self;

    fn add(self, input: Self) -> VecThreeFloat {
        Self {
            x: self.x + input.x,
            y: self.y + input.y,
            z: self.z + input.z,
        }
    }
}

impl Mul<f64> for VecThreeFloat {
    type Output = Self;

    fn mul(self, input: f64) -> VecThreeFloat {
        Self {
            x: self.x * input,
            y: self.y * input,
            z: self.z * input,
        }
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

#[derive(Clone, Copy, Debug)]
pub struct VecFour {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl VecFour {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Color> for VecFour {
    fn from(input: Color) -> Self {
        Self {
            x: input.r as f64,
            y: input.g as f64,
            z: input.b as f64,
            w: input.a as f64,
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn vec_three_mul() {
        let ret = VecThreeFloat::new(5.0, 0.0, 0.0) * 1.0;
        assert_eq!(ret.x, 5.0);
        assert_eq!(ret.y, 0.0);
        assert_eq!(ret.z, 0.0);
    }
}
