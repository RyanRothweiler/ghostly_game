use crate::{error::*, model::*, vectors::*};

use std::path::Path;

pub fn load_file(file_path: &Path) -> Result<Model, Error> {
    let file_data = std::fs::read_to_string(file_path)?;
    load(&file_data)
}

pub fn load(input: &str) -> Result<Model, Error> {
    let mut model = Model::new();

    let mut tokenizer = Tokenizer::new(input);

    let mut vertices: Vec<VecThreeFloat> = vec![];
    let mut normals: Vec<VecThreeFloat> = vec![];
    let mut uvs: Vec<VecTwo> = vec![];
    let mut faces_count: u32 = 0;

    loop {
        let token = tokenizer.get_next_token()?;

        match token {
            Token::Vertex => {
                let x: f64 = tokenizer.get_next_token()?.require_float()?;
                let y: f64 = tokenizer.get_next_token()?.require_float()?;
                let z: f64 = tokenizer.get_next_token()?.require_float()?;
                vertices.push(VecThreeFloat::new(x, y, z));
            }
            Token::Texture => {
                let u: f64 = tokenizer.get_next_token()?.require_float()?;
                let v: f64 = tokenizer.get_next_token()?.require_float()?;

                // flip the V? This is probably graphics backend dependent.
                uvs.push(VecTwo::new(u, -v));
            }
            Token::Normal => {
                let x: f64 = tokenizer.get_next_token()?.require_float()?;
                let y: f64 = tokenizer.get_next_token()?.require_float()?;
                let z: f64 = tokenizer.get_next_token()?.require_float()?;
                normals.push(VecThreeFloat::new(x, y, z));
            }
            Token::Face => {
                loop {
                    // This assumes three components. Which isn't a safe assumption.
                    // But it is for me right now.

                    model.indices.push(faces_count);
                    faces_count = faces_count + 1;

                    // first token is the face index
                    match tokenizer.get_next_token()? {
                        Token::Float(v) => {
                            // obj vertices start at 1. there is no 0 index.
                            // but ogl indices start at 0.
                            let i = v as u32 - 1;
                            model.vertices.push(vertices[i as usize]);
                        }

                        // If not a float then that is an error
                        _ => {
                            return Err(Error::ObjTokenParsingError);
                        }
                    };

                    // slash token
                    tokenizer.get_next_token()?;

                    // second float is uv index
                    match tokenizer.get_next_token()? {
                        Token::Float(v) => {
                            // obj vertices start at 1. there is no 0 index.
                            // but ogl indices start at 0.
                            let i = v as u32 - 1;
                            model.uvs.push(uvs[i as usize]);
                        }

                        // If not a float then that is an error
                        _ => {
                            return Err(Error::ObjTokenParsingError);
                        }
                    };

                    // slash token
                    tokenizer.get_next_token()?;

                    // third float is normal index
                    match tokenizer.get_next_token()? {
                        Token::Float(v) => {
                            // obj vertices start at 1. there is no 0 index.
                            // but ogl indices start at 0.
                            let i = v as u32 - 1;
                            model.normals.push(normals[i as usize]);
                        }

                        // If not a float then that is an error
                        _ => {
                            return Err(Error::ObjTokenParsingError);
                        }
                    };

                    // If next token is a number, then continue looping to grab that index.
                    // Otheriwse its something else so break out of loop
                    match tokenizer.peek_token()? {
                        Token::Float(_v) => (),
                        _ => break,
                    }
                }
            }
            Token::End => break,
            _ => continue,
        }
    }

    model.calculate_tangents();

    Ok(model)
}

#[derive(PartialEq, Debug)]
enum Token {
    Comment(String),
    Float(f64),
    Mttlib(String),
    Usemtl(String),
    Object(String),
    Vertex,
    Normal,
    Texture,
    SmoothShading,
    Face,
    End,
    ForwardSlash,
}

impl Token {
    // Returns the float if the token is a float, othewise returns error
    pub fn require_float(&self) -> Result<f64, Error> {
        match self {
            Token::Float(v) => Ok(*v),
            _ => return Err(Error::ObjTokenParsingError),
        }
    }
}

struct Tokenizer {
    pub data: Vec<char>,
    // pub data: String,
    pub index: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            data: input.chars().collect(),
            index: 0,
        }
    }

    // returns the new char
    pub fn advance(&mut self) -> Option<char> {
        if self.index >= self.data.len() {
            return None;
        }

        let c: char = self.data[self.index];
        self.index = self.index + 1;
        return Some(c);
    }

    pub fn move_until(&mut self, is_finished: impl Fn(char) -> bool) {
        loop {
            match self.advance() {
                Some(v) => {
                    if is_finished(v) {
                        self.index = self.index - 1;
                        return;
                    }
                }
                None => return,
            }
        }
    }

    pub fn move_to_num(&mut self) {
        self.move_until(|c| c.is_numeric());
    }

    pub fn move_to_char(&mut self, ct: char) {
        self.move_until(|c| c == ct);
    }

    pub fn move_to_line_end(&mut self) {
        self.move_until(|c| c == '\n');
    }

    pub fn get_current(&self) -> Option<char> {
        if self.index >= self.data.len() {
            return None;
        } else {
            return Some(self.data[self.index]);
        }
    }

    pub fn extract(&self, start: usize, end: usize) -> Option<String> {
        if start == end {
            return None;
        }
        if start > end {
            return None;
        }
        if start > self.data.len() || end > self.data.len() {
            return None;
        }

        let mut ret = String::new();
        let sub = &self.data[start..end];
        for c in sub {
            ret.push(*c);
        }
        Some(ret)
    }

    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        // move until we find a character we recognize
        loop {
            let c: char = match self.get_current() {
                Some(v) => v,
                None => return Ok(Token::End),
            };

            if self.starts_with("# ") {
                // found comment

                // don't include the # char
                self.advance();

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };
                return Ok(Token::Comment(sub.trim().to_string()));
            } else if self.starts_with("f ") {
                self.index = self.index + 1;
                return Ok(Token::Face);
            } else if self.starts_with("s ") {
                self.index = self.index + 1;
                return Ok(Token::SmoothShading);
            } else if self.starts_with("vt ") {
                self.index = self.index + 2;
                return Ok(Token::Texture);
            } else if self.starts_with("vn ") {
                self.index = self.index + 2;
                return Ok(Token::Normal);
            } else if self.starts_with("v ") {
                self.index = self.index + 1;
                return Ok(Token::Vertex);
            } else if self.starts_with("/") {
                self.index = self.index + 1;
                return Ok(Token::ForwardSlash);
            } else if self.starts_with("o ") {
                self.advance();

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };

                return Ok(Token::Object(sub.trim().to_string()));
            } else if self.starts_with("usemtl ") {
                self.index += 7;

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };
                return Ok(Token::Usemtl(sub.trim().to_string()));
            } else if self.starts_with("mttlib ") {
                self.index += 7;

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };
                return Ok(Token::Mttlib(sub.trim().to_string()));
            } else if c.is_numeric() || c == '-' {
                let mut neg = 1.0;
                if c == '-' {
                    self.advance();
                    neg = -1.0;
                }

                // found number
                self.move_to_num();
                let num_start = self.index;

                self.move_until(|c| !c.is_numeric() && c != '.');

                let num_end = self.index;

                // Tokenizer didn't move, so at end of string
                if num_start == num_end {
                    return Ok(Token::End);
                }

                let sub = match self.extract(num_start, num_end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };
                let num: f64 = sub.parse()?;
                return Ok(Token::Float(num * neg));
            } else {
                //unknown character. Continue past it.
                self.advance();
            }
        }
    }

    pub fn peek_token(&mut self) -> Result<Token, Error> {
        let orig = self.index;
        let ret = self.get_next_token();
        self.index = orig;
        return ret;
    }

    pub fn starts_with(&self, input: &str) -> bool {
        let mut i: usize = 0;
        for c in input.chars() {
            if c != self.data[i + self.index] {
                return false;
            }
            i = i + 1;
        }
        return true;
    }
}

mod test {
    use super::*;

    #[test]
    fn move_until() {
        let input = "aldf eee, 1 1.0 1.00001 ::::, 100.123 123 ";
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.move_until(|c| c == 'e');

        assert_eq!(tokenizer.get_current(), Some('e'));

        tokenizer.move_until(|c| c == ',');
        assert_eq!(tokenizer.get_current(), Some(','));

        tokenizer.move_until(|c| c == '!');
        assert_eq!(tokenizer.get_current(), None);

        tokenizer.move_until(|c| c == '!');
        assert_eq!(tokenizer.get_current(), None);
    }

    #[test]
    fn move_to_num() {
        let input = "ald eee, 1 1.0 1.00001 ::::, 100.123 123 ";
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.move_to_num();

        assert_eq!(tokenizer.get_current(), Some('1'));
    }

    #[test]
    fn get_token_float() {
        let input = "ald eee, 1 1.0 1.00001 ::::, 100.123 123what 123 hey -5.1 99";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.00001));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(100.123));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(123.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(123.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(-5.1));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(99.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }

    #[test]
    fn get_token_comment() {
        let input = "# comment here \n # what more comment";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::Comment("comment here".to_string())
        );
        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::Comment("what more comment".to_string())
        );
    }

    #[test]
    fn get_token_mttlib() {
        let input = "mttlib cube.mtl \n mttlib one more";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::Mttlib("cube.mtl".to_string())
        );
        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::Mttlib("one more".to_string())
        );
    }

    #[test]
    fn get_token_usemtl() {
        let input = "usemtl Material";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::Usemtl("Material".to_string())
        );
    }

    #[test]
    fn get_token_object() {
        let input = "o Cube";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::Object("Cube".to_string())
        );
    }

    #[test]
    fn get_token_vertex() {
        let input = "v 1.000000 -1.000000 -1.000000";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Vertex);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(-1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(-1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }

    #[test]
    fn get_token_normal() {
        let input = "vn 1.000000 -1.000000 -1.000000";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Normal);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(-1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(-1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }

    #[test]
    fn get_token_texture() {
        let input = "vt 0.625000 0.500000";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Texture);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(0.625));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(0.5));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }

    #[test]
    fn starts_with() {
        let input = "heyo 12 dude what";
        let tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.starts_with("he"), true);
        assert_eq!(tokenizer.starts_with("ee"), false);
        assert_eq!(tokenizer.starts_with("hey"), true);
    }

    #[test]
    fn get_token_face() {
        let input = "f 1/1/1 5/2/1 7/3/1 3/4/1";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Face);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::ForwardSlash);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::ForwardSlash);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(5.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::ForwardSlash);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(2.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::ForwardSlash);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
    }
}
