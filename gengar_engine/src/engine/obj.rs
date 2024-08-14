use crate::engine::error::*;
use crate::engine::model::*;
use crate::engine::vectors::*;

use std::path::Path;

pub fn load_file(file_path: &Path) -> Result<Model, Error> {
    let file_data = std::fs::read_to_string(file_path)?;
    load(&file_data)
}

pub fn load(input: &str) -> Result<Model, Error> {
    let mut model = Model::new();

    let mut tokenizer = Tokenizer::new(input);

    loop {
        let token = tokenizer.get_next_token()?;

        match token {
            Token::Vertex => {
                let x: f64 = match tokenizer.get_next_token()? {
                    Token::Float(v) => v,
                    _ => return Err(Error::ObjTokenParsingError),
                };
                let y: f64 = match tokenizer.get_next_token()? {
                    Token::Float(v) => v,
                    _ => return Err(Error::ObjTokenParsingError),
                };
                let z: f64 = match tokenizer.get_next_token()? {
                    Token::Float(v) => v,
                    _ => return Err(Error::ObjTokenParsingError),
                };

                let vertex = VecThreeFloat::new(x, y, z);
                model.vertices.push(vertex);
            }
            Token::End => break,
            _ => continue,
        }
    }

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
    Tangent,
    SmoothShading,
    Face,
    End,
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

        let sub = &self.data[start..end];
        let sub: Vec<char> = sub.iter().cloned().collect();
        let sub: String = sub.into_iter().collect();
        Some(sub)
    }

    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        // move until we find a character we recognize
        loop {
            let c: char = match self.get_current() {
                Some(v) => v,
                None => return Ok(Token::End),
            };

            // convert back to string
            let current = &self.data[self.index..self.data.len()];
            let current: Vec<char> = current.iter().cloned().collect();
            let current: String = current.into_iter().collect();

            if current.starts_with("# ") {
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
            } else if current.starts_with("f ") {
                self.index = self.index + 1;
                return Ok(Token::Face);
            } else if current.starts_with("s ") {
                self.index = self.index + 1;
                return Ok(Token::SmoothShading);
            } else if current.starts_with("vt ") {
                self.index = self.index + 2;
                return Ok(Token::Tangent);
            } else if current.starts_with("vn ") {
                self.index = self.index + 2;
                return Ok(Token::Normal);
            } else if current.starts_with("v ") {
                self.index = self.index + 1;
                return Ok(Token::Vertex);
            } else if current.starts_with("o ") {
                self.advance();

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };

                return Ok(Token::Object(sub.trim().to_string()));
            } else if current.starts_with("usemtl ") {
                self.index += 7;

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };
                return Ok(Token::Usemtl(sub.trim().to_string()));
            } else if current.starts_with("mttlib ") {
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
    fn get_token_tangent() {
        let input = "vt 0.625000 0.500000";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Tangent);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(0.625));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(0.5));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }

    #[test]
    fn get_token_face() {
        let input = "f 1/1/1 5/2/1 7/3/1 3/4/1";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Face);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(5.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(2.0));
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(1.0));
    }

    #[test]
    fn model_vertex() {
        let input = "v 1.000000 1.000000 -1.000000 \n v 1.000000 -1.000000 -1.000000 \n v 1.000000 1.000000 1.000000";
        let model = load(input).unwrap();

        assert_eq!(model.vertices.len(), 3);

        assert_eq!(model.vertices[0].x, 1.0);
        assert_eq!(model.vertices[0].y, 1.0);
        assert_eq!(model.vertices[0].z, -1.0);

        assert_eq!(model.vertices[1].x, 1.0);
        assert_eq!(model.vertices[1].y, -1.0);
        assert_eq!(model.vertices[1].z, -1.0);

        assert_eq!(model.vertices[2].x, 1.0);
        assert_eq!(model.vertices[2].y, 1.0);
        assert_eq!(model.vertices[2].z, 1.0);
    }
}
