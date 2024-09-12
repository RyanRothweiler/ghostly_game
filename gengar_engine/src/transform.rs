use crate::vectors::*;

pub struct Transform {
    pub position: VecThreeFloat,
    pub scale: VecThreeFloat,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: VecThreeFloat::new_zero(),
            scale: VecThreeFloat::new(1.0, 1.0, 1.0),
        }
    }
}
