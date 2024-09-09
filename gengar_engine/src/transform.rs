use crate::vectors::*;

pub struct Transform {
    pub position: VecThreeFloat,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: VecThreeFloat::new_zero(),
        }
    }
}
