use crate::engine::matricies::matrix_four_four::*;
use crate::engine::transform::*;
use crate::engine::vectors::*;

pub struct Camera {
    pub transform: Transform,
    pub view_mat: M44,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            transform: Transform::new(),
            view_mat: M44::new_identity(),
        }
    }

    pub fn update_matricies(&mut self) {
        // View matrix
        let inv_pos = VecThreeFloat::new(
            -self.transform.position.x,
            -self.transform.position.y,
            -self.transform.position.z,
        );
        self.view_mat = M44::new_identity();
        self.view_mat.translate(inv_pos);
    }
}
