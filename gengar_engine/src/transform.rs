use crate::vectors::*;

pub struct Transform {
    pub parent: Option<usize>,

    pub position: VecThreeFloat,
    pub scale: VecThreeFloat,
    pub rotation: VecThreeFloat,

    pub local_position: VecThreeFloat,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: VecThreeFloat::new_zero(),
            rotation: VecThreeFloat::new_zero(),
            scale: VecThreeFloat::new(1.0, 1.0, 1.0),

            local_position: VecThreeFloat::new_zero(),
            parent: None,
        }
    }

    pub fn update_all(transforms: &mut Vec<Self>) {
        for i in 0..transforms.len() {
            let parent_id_opt: Option<usize> = transforms[i].parent;
            let parent_pos: VecThreeFloat = match parent_id_opt {
                Some(pid) => transforms[pid].position,

                None => VecThreeFloat::new_zero(),
            };

            transforms[i].position = transforms[i].local_position + parent_pos;
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

        assert_eq!(
            VecThreeFloat::close_enough(
                &transforms[0].position,
                &VecThreeFloat::new(10.0, 0.0, 0.0)
            ),
            true
        );

        assert_eq!(
            VecThreeFloat::close_enough(
                &transforms[1].position,
                &VecThreeFloat::new(15.0, 0.0, 0.0)
            ),
            true
        );
    }
}
