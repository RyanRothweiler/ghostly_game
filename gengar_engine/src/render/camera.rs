use crate::{matricies::matrix_four_four::*, transform::*, vectors::*};

pub enum ProjectionType {
    Perspective(ProjectionInfo),
}

pub struct Camera {
    pub transform: Transform,
    pub view_mat: M44,
    pub projection_mat: M44,
    pub projection_type: ProjectionType,
    pub resolution: VecTwo,
    pub near_plane: f64,
    pub far_plane: f64,
    pub fov: f64,
}

pub struct ProjectionInfo {
    pub focal_length: f64,
}

impl Camera {
    pub fn new(projection_type: ProjectionType, resolution: VecTwo) -> Self {
        Camera {
            transform: Transform::new(),
            view_mat: M44::new_identity(),
            projection_mat: M44::new_identity(),

            resolution,
            projection_type,

            near_plane: 0.1,
            far_plane: 10.0,
            fov: 0.0,
        }
    }

    pub fn update_matricies(&mut self) {
        match &self.projection_type {
            ProjectionType::Perspective(info) => {
                let aspect: f64 = self.resolution.x / self.resolution.y;

                let a = 1.0;
                let b = aspect;
                let c = info.focal_length;

                let d = (self.near_plane + self.far_plane) / (self.near_plane - self.far_plane);
                let e =
                    (2.0 * self.far_plane * self.near_plane) / (self.near_plane - self.far_plane);

                self.projection_mat = M44::new_identity();
                self.projection_mat.set(0, 0, a * c);
                self.projection_mat.set(1, 1, b * c);
                self.projection_mat.set(2, 2, d);
                self.projection_mat.set(3, 2, e);
                self.projection_mat.set(2, 3, -1.0);
            }
        }

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
