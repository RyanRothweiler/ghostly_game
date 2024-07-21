// Render backend independent calls
/*
pub struct RenderApi {
    pub make_shader_program: fn() -> i32,
}
*/

pub trait RenderApi {
    fn make_shader_program(&self) -> i32;
}

/*
impl RenderApi {
    pub fn make_program(&self, vert_shader: &str, frag_shader: &str) -> i32 {
        let prog_id = (self.create_shader)();

        return prog_id;
    }
}
*/

// pub fn make_program(RenderApi) {}
