use crate::engine::vectors::*;

#[derive(Debug, Clone, Copy)]
pub struct M44 {
    pub elements: [f64; 16],
}

impl M44 {
    pub fn new_empty() -> Self {
        M44 {
            elements: [0.0; 16],
        }
    }

    pub fn new_identity() -> Self {
        let mut ret = M44::new_empty();

        ret.set(0, 0, 1.0);
        ret.set(1, 1, 1.0);
        ret.set(2, 2, 1.0);
        ret.set(3, 3, 1.0);

        return ret;
    }

    pub fn new_translation(translation: VecThreeFloat) -> Self {
        let mut trans_mat = M44::new_identity();

        trans_mat.set(3, 0, translation.x);
        trans_mat.set(3, 1, translation.y);
        trans_mat.set(3, 2, translation.z);

        return trans_mat;
    }

    pub fn new_scale(scale: VecThreeFloat) -> Self {
        let mut trans_mat = M44::new_identity();

        trans_mat.set(0, 0, scale.x);
        trans_mat.set(1, 1, scale.y);
        trans_mat.set(2, 2, scale.z);

        return trans_mat;
    }

    pub fn new_rotation_x(rot: f64) -> Self {
        let mut trans_mat = M44::new_identity();
        let c = f64::cos(rot);
        let s = f64::sin(rot);

        trans_mat.set(1, 1, c);
        trans_mat.set(2, 1, s);
        trans_mat.set(1, 2, -s);
        trans_mat.set(2, 2, c);

        return trans_mat;
    }

    pub fn new_rotation_y(rot: f64) -> Self {
        let mut trans_mat = M44::new_identity();
        let c = f64::cos(rot);
        let s = f64::sin(rot);

        trans_mat.set(0, 0, c);
        trans_mat.set(2, 0, -s);
        trans_mat.set(0, 2, s);
        trans_mat.set(2, 2, c);

        return trans_mat;
    }

    pub fn new_rotation_z(rot: f64) -> Self {
        let mut trans_mat = M44::new_identity();
        let c = f64::cos(rot);
        let s = f64::sin(rot);

        trans_mat.set(0, 0, c);
        trans_mat.set(1, 0, -s);
        trans_mat.set(0, 1, s);
        trans_mat.set(1, 1, c);

        return trans_mat;
    }

    pub fn multiply(a: &M44, b: &M44) -> Self {
        let mut ret = M44::new_empty();

        for x in 0..4 {
            for y in 0..4 {
                let mut val: f64 = 0.0;

                for i in 0..4 {
                    val = val + (a.get(i, y) * b.get(x, i));
                }

                ret.set(x, y, val);
            }
        }

        return ret;
    }

    pub fn apply_vec_three(mat: &M44, vec: &VecThreeFloat) -> VecThreeFloat {
        let mut ret = VecThreeFloat::new_zero();
        let w = 1.0;

        ret.x = (mat.get(0, 0) * vec.x)
            + (mat.get(1, 0) * vec.y)
            + (mat.get(2, 0) * vec.z)
            + (mat.get(3, 0) * w);

        ret.y = (mat.get(0, 1) * vec.x)
            + (mat.get(1, 1) * vec.y)
            + (mat.get(2, 1) * vec.z)
            + (mat.get(3, 1) * w);

        ret.z = (mat.get(0, 2) * vec.x)
            + (mat.get(1, 2) * vec.y)
            + (mat.get(2, 2) * vec.z)
            + (mat.get(3, 2) * w);

        return ret;
    }

    pub fn set(&mut self, x: usize, y: usize, val: f64) {
        let i: usize = (4 * x) + y;
        self.elements[i] = val;
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        let i: usize = (4 * x) + y;
        return self.elements[i];
    }

    pub fn transpose(&self) -> Self {
        let mut ret = M44::new_empty();

        for x in 0..4 {
            for y in 0..4 {
                ret.set(x, y, self.get(y, x));
            }
        }

        return ret;
    }

    pub fn translate(&mut self, translation: VecThreeFloat) {
        let trans = M44::new_translation(translation);
        let result = M44::multiply(self, &trans);
        for x in 0..self.elements.len() {
            self.elements[x] = result.elements[x];
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn set_get() {
        let mut mat = M44::new_empty();

        mat.set(1, 1, 5.0);
        assert_eq!(mat.get(1, 1), 5.0);

        mat.set(0, 0, 1.0);
        assert_eq!(mat.get(0, 0), 1.0);

        mat.set(3, 3, 123.5);
        assert_eq!(mat.get(3, 3), 123.5);

        mat.set(1, 3, 123.5);
        assert_eq!(mat.get(1, 3), 123.5);
    }

    #[test]
    fn multiply() {
        let mut a = M44::new_empty();
        let mut b = M44::new_empty();

        let mut i: f64 = 1.0;
        for y in 0..4 {
            for x in 0..4 {
                a.set(x, y, i);
                b.set(x, y, i);
                i = i + 1.0;
            }
        }

        let ret = M44::multiply(&a, &b);
        println!("{:?}", ret);
        assert_eq!(ret.get(0, 0), 90.0);
        assert_eq!(ret.get(1, 0), 100.0);
        assert_eq!(ret.get(2, 0), 110.0);
        assert_eq!(ret.get(3, 0), 120.0);

        assert_eq!(ret.get(0, 1), 202.0);
        assert_eq!(ret.get(1, 1), 228.0);
        assert_eq!(ret.get(2, 1), 254.0);
        assert_eq!(ret.get(3, 1), 280.0);

        assert_eq!(ret.get(0, 2), 314.0);
        assert_eq!(ret.get(1, 2), 356.0);
        assert_eq!(ret.get(2, 2), 398.0);
        assert_eq!(ret.get(3, 2), 440.0);

        assert_eq!(ret.get(0, 3), 426.0);
        assert_eq!(ret.get(1, 3), 484.0);
        assert_eq!(ret.get(2, 3), 542.0);
        assert_eq!(ret.get(3, 3), 600.0);
    }

    #[test]
    fn transpose() {
        let mut a = M44::new_empty();
        let mut i: f64 = 1.0;

        for y in 0..4 {
            for x in 0..4 {
                a.set(x, y, i);
                i = i + 1.0;
            }
        }

        assert_eq!(a.get(0, 0), 1.0);
        assert_eq!(a.get(1, 0), 2.0);
        assert_eq!(a.get(2, 0), 3.0);
        assert_eq!(a.get(3, 0), 4.0);

        let b = a.transpose();

        assert_eq!(b.get(0, 0), 1.0);
        assert_eq!(b.get(1, 0), 5.0);
        assert_eq!(b.get(2, 0), 9.0);
        assert_eq!(b.get(3, 0), 13.0);

        assert_eq!(b.get(0, 1), 2.0);
        assert_eq!(b.get(1, 1), 6.0);
        assert_eq!(b.get(2, 1), 10.0);
        assert_eq!(b.get(3, 1), 14.0);

        assert_eq!(b.get(0, 2), 3.0);
        assert_eq!(b.get(1, 2), 7.0);
        assert_eq!(b.get(2, 2), 11.0);
        assert_eq!(b.get(3, 2), 15.0);

        assert_eq!(b.get(0, 3), 4.0);
        assert_eq!(b.get(1, 3), 8.0);
        assert_eq!(b.get(2, 3), 12.0);
        assert_eq!(b.get(3, 3), 16.0);
    }

    #[test]
    fn apply_vec_three() {
        let mut a = M44::new_identity();
        a.translate(VecThreeFloat::new(1.0, 2.0, 4.0));

        let point = VecThreeFloat::new(2.0, 1.5, 123.123);
        let res = M44::apply_vec_three(&a, &point);

        assert_eq!(res.x, 3.0);
        assert_eq!(res.y, 3.5);
        assert_eq!(res.z, 127.123);
    }
}
