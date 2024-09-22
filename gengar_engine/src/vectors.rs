use crate::color::*;
use std::ops::{Add, Mul, Sub};

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

    pub fn cross(a: Self, b: Self) -> Self {
        let mut ret = VecThreeFloat::new_zero();
        ret.x = (a.y * b.z) - (a.z * b.y);
        ret.y = (a.z * b.x) - (a.x * b.z);
        ret.z = (a.x * b.y) - (a.y * b.x);
        ret
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.x = self.x / len;
        self.y = self.y / len;
        self.z = self.z / len;
    }

    pub fn close_enough(a: &Self, b: &Self) -> bool {
        let decs = 100.0;
        ((a.x * decs) as i64 == (b.x * decs) as i64)
            && ((a.y * decs) as i64 == (b.y * decs) as i64)
            && ((a.z * decs) as i64 == (b.z * decs) as i64)
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

impl Sub for VecThreeFloat {
    type Output = Self;

    fn sub(self, input: Self) -> VecThreeFloat {
        Self {
            x: self.x - input.x,
            y: self.y - input.y,
            z: self.z - input.z,
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

    #[test]
    fn vec_three_normalize() {
        let mut ret = VecThreeFloat::new(5.0, 1.0, 2.5);
        ret.normalize();

        assert_eq!((ret.x * 100.0) as i32, 88);
        assert_eq!((ret.y * 100.0) as i32, 17);
        assert_eq!((ret.z * 100.0) as i32, 44);
    }

    #[test]
    fn vec_three_cross() {
        let a = VecThreeFloat::new(5.0, 1.0, 2.5);
        let b = VecThreeFloat::new(1.0, 0.0, -10.5);
        let c = VecThreeFloat::cross(a, b);

        assert_eq!(c.x, -10.5);
        assert_eq!(c.y, 55.0);
        assert_eq!(c.z, -1.0);
    }

    #[test]
    fn vec_three_equal() {
        let a = VecThreeFloat::new(5.0, 0.0, 0.0);
        let b = VecThreeFloat::new(5.0, 0.0, 0.0);
        let c = VecThreeFloat::new(0.0, 1.0, 0.0);

        assert_eq!(VecThreeFloat::close_enough(&a, &b), true);
        assert_eq!(VecThreeFloat::close_enough(&b, &a), true);
        assert_eq!(VecThreeFloat::close_enough(&a, &c), false);
    }
}
