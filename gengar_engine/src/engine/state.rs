use crate::engine::render::camera::*;
use crate::engine::render::render_command::*;
use crate::engine::render::shader::*;
use crate::engine::render::vao::*;

pub struct State {
    pub basic_shader: Shader,
    pub cube: Vao,

    pub frame: i64,

    pub render_commands: Vec<RenderCommand>,

    pub camera: Camera,
}

impl State {
    pub fn new() -> Self {
        State {
            basic_shader: Shader::new_empty(),
            cube: Vao::new_empty(),
            frame: 0,
            render_commands: vec![],
            camera: Camera::new(),
        }
    }
}

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
    pub mouse_left: ButtonState,
    pub mouse_right: ButtonState,
}

impl Input {
    pub fn new() -> Self {
        Input {
            mouse_left: ButtonState::new(),
            mouse_right: ButtonState::new(),
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
