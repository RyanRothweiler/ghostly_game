use crate::{
    ascii::*, color::*, debug::*, matricies::matrix_four_four::*, state::Input, transform::*,
    vectors::*,
};

pub enum ProjectionType {
    Perspective(ProjectionInfo),
}

pub struct Camera {
    pub euler_rotation: VecThreeFloat,
    pub forward: VecThreeFloat,

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

            euler_rotation: VecThreeFloat::new_zero(),
            forward: VecThreeFloat::new_zero(),

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

        let up = VecThreeFloat::new(0.0, 1.0, 0.0);

        /*
        // broken fly cam attempt
        {
            let rot_y: M44 = M44::new_rotation_y(self.euler_rotation.x * -1.0);
            self.forward = M44::apply_vec_three(&rot_y, &VecThreeFloat::new(0.0, 0.0, 1.0));
            let rot_z: M44 = M44::new_rotation_x(self.euler_rotation.y * -1.0);
            self.forward = M44::apply_vec_three(&rot_z, &self.forward);

            let look_target_position = self.transform.position + (self.forward * -1.0);

            // z
            let mut z_axis = self.transform.position - look_target_position;
            z_axis.normalize();

            // x
            let mut x_axis = VecThreeFloat::cross(up, z_axis);
            x_axis.normalize();

            // y
            let _y_axis = VecThreeFloat::cross(z_axis, x_axis);

        }
        */
        draw_sphere(self.forward, 0.1, Color::new(0.0, 1.0, 0.0, 1.0));

        let target_pos = self.forward;

        let mut cam_dir = self.transform.position - target_pos;
        cam_dir.normalize();

        let mut cam_right = VecThreeFloat::cross(up, cam_dir);
        cam_right.normalize();

        let cam_up = VecThreeFloat::cross(cam_dir, cam_right);

        // View matrix
        self.view_mat = M44::new_identity();

        let inv_pos = VecThreeFloat::new(
            -self.transform.position.x,
            -self.transform.position.y,
            -self.transform.position.z,
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

        /*
        self.view_mat.set(0, 0, x_axis.x);
        self.view_mat.set(0, 1, x_axis.y);
        self.view_mat.set(0, 2, x_axis.z);

        self.view_mat.set(1, 0, y_axis.x);
        self.view_mat.set(1, 1, y_axis.y);
        self.view_mat.set(1, 2, y_axis.z);

        self.view_mat.set(2, 0, z_axis.x);
        self.view_mat.set(2, 1, z_axis.y);
        self.view_mat.set(2, 2, z_axis.z);
        */
    }

    // Control the camera as a fly-cam
    // Mouse for rotation and wasd for camera relative movement
    pub fn move_fly_(&mut self, speed: f64, input: &Input) {
        /*
        if input.mouse_right.pressing {
            // println!("{:?}", self.euler_rotation);
            self.euler_rotation.x = self.euler_rotation.x + (input.mouse_pos_delta.x * 0.001);
            self.euler_rotation.y = self.euler_rotation.y + (input.mouse_pos_delta.y * 0.001);
        }

        println!("{:?}", self.transform.position);

        let right = VecThreeFloat::cross(self.forward, VecThreeFloat::new(0.0, 1.0, 0.0));
        let up = VecThreeFloat::cross(self.forward, VecThreeFloat::new(1.0, 0.0, 0.0));

        if input.keyboard[ASCII_A].pressing {
            self.transform.position = self.transform.position + (right * speed);
        }
        if input.keyboard[ASCII_D].pressing {
            self.transform.position = self.transform.position - (right * speed);
        }
        if input.keyboard[ASCII_S].pressing {
            self.transform.position = self.transform.position + (self.forward * speed);
        }
        if input.keyboard[ASCII_W].pressing {
            self.transform.position = self.transform.position - (self.forward * speed);
        }
        if input.keyboard[ASCII_Q].pressing {
            self.transform.position = self.transform.position + (up * speed);
        }
        if input.keyboard[ASCII_E].pressing {
            self.transform.position = self.transform.position - (up * speed);
        }
        */

        if input.keyboard[ASCII_A].pressing {
            self.forward.x = self.forward.x - speed;
        }
        if input.keyboard[ASCII_D].pressing {
            self.forward.x = self.forward.x + speed;
        }
        if input.keyboard[ASCII_S].pressing {
            self.forward.y = self.forward.y - speed;
        }
        if input.keyboard[ASCII_W].pressing {
            self.forward.y = self.forward.y + speed;
        }

        self.update_matricies();
    }
}
