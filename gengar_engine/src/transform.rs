use crate::{matricies::matrix_four_four::*, vectors::*};

pub struct Transform {
    pub parent: Option<usize>,

    pub scale: VecThreeFloat,

    pub local_position: VecThreeFloat,
    pub local_rotation: VecThreeFloat,

    pub global_matrix: M44,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            scale: VecThreeFloat::new(1.0, 1.0, 1.0),

            local_position: VecThreeFloat::new_zero(),
            local_rotation: VecThreeFloat::new_zero(),

            parent: None,
            global_matrix: M44::new_identity(),
        }
    }

    pub fn update_all(transforms: &mut Vec<Self>) {
        for i in 0..transforms.len() {
            let parent_id_opt: Option<usize> = transforms[i].parent;

            let parent_matrix: M44 = match parent_id_opt {
                Some(pid) => transforms[pid].global_matrix,

                None => M44::new_identity(),
            };

            let self_trans = &mut transforms[i];

            // apply local transformations
            let local_translation = M44::new_translation(self_trans.local_position);
            let local_rotation_x = M44::new_rotation_x(self_trans.local_rotation.x);
            let local_rotation_y = M44::new_rotation_y(self_trans.local_rotation.y);
            let local_rotation_z = M44::new_rotation_z(self_trans.local_rotation.z);

            self_trans.global_matrix = M44::new_identity();

            self_trans.global_matrix = M44::multiply(&self_trans.global_matrix, &local_rotation_x);
            self_trans.global_matrix = M44::multiply(&self_trans.global_matrix, &local_rotation_y);
            self_trans.global_matrix = M44::multiply(&self_trans.global_matrix, &local_rotation_z);

            self_trans.global_matrix = M44::multiply(&self_trans.global_matrix, &local_translation);

            // apply parent transformations
            self_trans.global_matrix = M44::multiply(&parent_matrix, &self_trans.global_matrix);
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn update_all() {
        let mut transforms: Vec<Transform> = vec![];
        transforms.push(Transform::new());
        transforms.push(Transform::new());

        transforms[0].local_position = VecThreeFloat::new(10.0, 0.0, 0.0);

        transforms[1].parent = Some(0);
        transforms[1].local_position = VecThreeFloat::new(5.0, 0.0, 0.0);

        Transform::update_all(&mut transforms);

        let origin = VecThreeFloat::new_zero();
        let zero_pos = M44::apply_vec_three(&transforms[0].global_matrix, &origin);
        let one_pos = M44::apply_vec_three(&transforms[1].global_matrix, &origin);

        assert_eq!(
            VecThreeFloat::close_enough(&zero_pos, &VecThreeFloat::new(10.0, 0.0, 0.0)),
            true
        );

        assert_eq!(
            VecThreeFloat::close_enough(&one_pos, &VecThreeFloat::new(15.0, 0.0, 0.0)),
            true
        );
    }
}
