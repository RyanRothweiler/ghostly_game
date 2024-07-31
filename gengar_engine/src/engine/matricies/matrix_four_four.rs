use crate::engine::vectors::*;

#[derive(Debug)]
struct MatrixFourFour {
    elements: [f64; 16],
}

impl MatrixFourFour {
    fn new_empty() -> Self {
        MatrixFourFour {
            elements: [0.0; 16],
        }
    }

    fn new_identity() -> Self {
        let mut ret = MatrixFourFour::new_empty();

        ret.set(0, 0, 1.0);
        ret.set(1, 1, 1.0);
        ret.set(2, 2, 1.0);
        ret.set(3, 3, 1.0);

        return ret;
    }

    fn new_translation(&mut self, translation: VecThreeFloat) -> Self {
        let mut trans_mat = MatrixFourFour::new_identity();

        trans_mat.set(3, 0, translation.x);
        trans_mat.set(3, 1, translation.y);
        trans_mat.set(3, 2, translation.z);

        return trans_mat;
    }

    fn new_scale(&mut self, scale: VecThreeFloat) -> Self {
        let mut trans_mat = MatrixFourFour::new_identity();

        trans_mat.set(0, 0, scale.x);
        trans_mat.set(1, 1, scale.y);
        trans_mat.set(2, 2, scale.z);

        return trans_mat;
    }

    fn new_rotation_x(&mut self, rot: f64) -> Self {
        let mut trans_mat = MatrixFourFour::new_identity();
        let c = f64::cos(rot);
        let s = f64::sin(rot);

        trans_mat.set(1, 1, c);
        trans_mat.set(2, 1, s);
        trans_mat.set(1, 2, -s);
        trans_mat.set(2, 2, c);

        return trans_mat;
    }

    fn new_rotation_y(&mut self, rot: f64) -> Self {
        let mut trans_mat = MatrixFourFour::new_identity();
        let c = f64::cos(rot);
        let s = f64::sin(rot);

        trans_mat.set(0, 0, c);
        trans_mat.set(2, 0, -s);
        trans_mat.set(0, 2, s);
        trans_mat.set(2, 2, c);

        return trans_mat;
    }

    fn new_rotation_z(&mut self, rot: f64) -> Self {
        let mut trans_mat = MatrixFourFour::new_identity();
        let c = f64::cos(rot);
        let s = f64::sin(rot);

        trans_mat.set(0, 0, c);
        trans_mat.set(1, 0, -s);
        trans_mat.set(0, 1, s);
        trans_mat.set(1, 1, c);

        return trans_mat;
    }

    fn multiply(a: &MatrixFourFour, b: &MatrixFourFour) -> Self {
        let mut ret = MatrixFourFour::new_empty();

        for x in 0..4 {
            for y in 0..4 {
                let mut val: f64 = 0.0;

                for i in 0..4 {
                    val = val + (a.get(i, y) * b.get(x, i));
                }

                // why x/y flipped? I'm not smart enough.
                ret.set(y, x, val);
            }
        }

        return ret;
    }

    fn set(&mut self, x: usize, y: usize, val: f64) {
        let i: usize = (4 * x) + y;
        self.elements[i] = val;
    }

    fn get(&self, x: usize, y: usize) -> f64 {
        let i: usize = (4 * x) + y;
        return self.elements[i];
    }
}

mod test {

    use super::*;

    #[test]
    fn set_get() {
        let mut mat = MatrixFourFour::new_empty();

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
        let mut a = MatrixFourFour::new_empty();
        let mut b = MatrixFourFour::new_empty();

        let mut i: f64 = 1.0;
        for x in 0..4 {
            for y in 0..4 {
                a.set(x, y, i);
                b.set(x, y, i);
                i = i + 1.0;
            }
        }

        let ret = MatrixFourFour::multiply(&a, &b);
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
}
