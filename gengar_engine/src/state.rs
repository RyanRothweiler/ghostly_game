use crate::{
    model::*,
    render::{camera::*, render_command::*, shader::*, vao::*},
    vectors::*,
};

pub struct State {
    pub window_resolution: VecTwo,

    pub basic_shader: Shader,
    pub model_sphere: Model,

    pub frame: i64,

    pub render_commands: Vec<RenderCommand>,

    pub camera: Camera,
}

impl State {
    pub fn new(window_resolution: VecTwo) -> Self {
        let mut state = State {
            basic_shader: Shader::new_empty(),
            frame: 0,
            render_commands: vec![],
            camera: Camera::new(
                ProjectionType::Perspective(ProjectionInfo { focal_length: 0.95 }),
                window_resolution,
            ),
            window_resolution,
            model_sphere: Model::new(),
        };

        state.camera.transform.position.z = 5.0;

        return state;
    }
}

#[derive(Copy, Clone)]
pub struct ButtonState {
    pub pressing: bool,
    pub on_press: bool,
    pub on_release: bool,
}

impl ButtonState {
    pub fn new() -> Self {
        ButtonState {
            pressing: false,
            on_press: false,
            on_release: false,
        }
    }

    pub fn update(&mut self, new_state: bool) {
        if new_state {
            self.on_release = false;

            if !self.pressing {
                self.on_press = true;
            } else {
                self.on_press = false;
            }
        } else {
            self.on_press = false;

            if self.pressing {
                self.on_release = true;
            } else {
                self.on_release = false;
            }
        }

        self.pressing = new_state;
    }
}

pub struct Input {
    pub mouse_pos: VecTwo,
    pub mouse_left: ButtonState,
    pub mouse_right: ButtonState,
    pub keyboard: [ButtonState; 128],
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse_left: ButtonState::new(),
            mouse_right: ButtonState::new(),
            mouse_pos: VecTwo::new(0.0, 0.0),
            keyboard: [ButtonState::new(); 128],
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn press_update() {
        let mut button = ButtonState::new();

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, true);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, true);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);

        button.update(true);

        assert_eq!(button.pressing, true);
        assert_eq!(button.on_press, true);
        assert_eq!(button.on_release, false);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, true);

        button.update(false);

        assert_eq!(button.pressing, false);
        assert_eq!(button.on_press, false);
        assert_eq!(button.on_release, false);
    }
}
