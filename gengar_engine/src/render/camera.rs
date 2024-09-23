use crate::{
    ascii::*, color::*, debug::*, matricies::matrix_four_four::*, state::Input, transform::*,
    vectors::*,
};

pub enum ProjectionType {
    Perspective(ProjectionInfo),
}

pub struct Camera {
    pub forward: VecThreeFloat,
    pub yaw: f64,
    pub pitch: f64,

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

            forward: VecThreeFloat::new(0.0, 0.0, 1.0),

            resolution,
            projection_type,

            near_plane: 0.1,
            far_plane: 10.0,
            fov: 0.0,

            yaw: 90.0,
            pitch: 0.0,
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

        // view matrix
        {
            let up = VecThreeFloat::new(0.0, 1.0, 0.0);

            // Cam yaw / pitch axis
            self.forward = VecThreeFloat::new_zero();
            self.forward.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
            self.forward.y = self.pitch.to_radians().sin();
            self.forward.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
            self.forward.normalize();

            let target_pos = self.transform.local_position + (self.forward * -1.0);

            let mut cam_dir = self.transform.local_position - target_pos;
            cam_dir.normalize();

            let mut cam_right = VecThreeFloat::cross(up, cam_dir);
            cam_right.normalize();

            let cam_up = VecThreeFloat::cross(cam_dir, cam_right);

            // Setup matrix
            self.view_mat = M44::new_identity();

            let inv_pos = VecThreeFloat::new(
                -self.transform.local_position.x,
                -self.transform.local_position.y,
                -self.transform.local_position.z,
            );

            self.view_mat.set(0, 0, cam_right.x);
            self.view_mat.set(1, 0, cam_right.y);
            self.view_mat.set(2, 0, cam_right.z);

            self.view_mat.set(0, 1, cam_up.x);
            self.view_mat.set(1, 1, cam_up.y);
            self.view_mat.set(2, 1, cam_up.z);

            self.view_mat.set(0, 2, cam_dir.x);
            self.view_mat.set(1, 2, cam_dir.y);
            self.view_mat.set(2, 2, cam_dir.z);

            self.view_mat.translate(inv_pos);
        }
    }

    // Control the camera as a fly-cam
    // Mouse for rotation and wasd for camera relative movement
    pub fn move_fly_(&mut self, mov_speed: f64, input: &Input) {
        if input.mouse_right.pressing {
            let sens = 0.08;
            self.yaw = self.yaw - (input.mouse_pos_delta.x * sens);
            self.pitch = self.pitch - (input.mouse_pos_delta.y * sens);
        }

        let mut right = VecThreeFloat::cross(self.forward, VecThreeFloat::new(0.0, 1.0, 0.0));
        right.normalize();

        let mut up = VecThreeFloat::cross(self.forward, right);
        up.normalize();

        if input.keyboard[ASCII_A].pressing {
            self.transform.local_position = self.transform.local_position + (right * mov_speed);
        }
        if input.keyboard[ASCII_D].pressing {
            self.transform.local_position = self.transform.local_position - (right * mov_speed);
        }
        if input.keyboard[ASCII_S].pressing {
            self.transform.local_position =
                self.transform.local_position + (self.forward * mov_speed);
        }
        if input.keyboard[ASCII_W].pressing {
            self.transform.local_position =
                self.transform.local_position - (self.forward * mov_speed);
        }
        if input.keyboard[ASCII_Q].pressing {
            self.transform.local_position = self.transform.local_position + (up * mov_speed);
        }
        if input.keyboard[ASCII_E].pressing {
            self.transform.local_position = self.transform.local_position - (up * mov_speed);
        }

        self.update_matricies();
    }
}
