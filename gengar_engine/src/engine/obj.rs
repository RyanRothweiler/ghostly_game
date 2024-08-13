use crate::engine::error::*;

use std::path::Path;

pub fn load(_file_path: &Path) -> Result<(), Error> {
    /*
    let file_data = std::fs::read_to_string(file_path)?;

    let mut _tokenizer = Tokenizer {
        data: file_data.chars().collect(),
        index: 0,
    };

    // let mut _current_token: Token = get_token(&mut tokenizer);
    */

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Token {
    Comment(String),
    Float(f64),
    Mttlib(String),
    // Identifier(String),
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

    pub fn get_next_token(&mut self) -> Token {
        // move until we find a character we recognize
        loop {
            let c: char = match self.get_current() {
                Some(v) => v,
                None => return Token::End,
            };

            // convert back to string
            let current = &self.data[self.index..self.data.len()];
            let current: Vec<char> = current.iter().cloned().collect();
            let current: String = current.into_iter().collect();

            if c == '#' {
                // found comment

                // don't include the # char
                self.advance();

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Token::End,
                };
                return Token::Comment(sub.trim().to_string());
            } else if current.starts_with("mttlib") {
                self.index += 6;

                let start = self.index;
                self.move_to_line_end();
                let end = self.index;

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Token::End,
                };
                return Token::Mttlib(sub.trim().to_string());
            } else if c.is_numeric() {
                // found number
                self.move_to_num();
                let num_start = self.index;

                self.move_until(|c| c.is_alphabetic() || c.is_whitespace());

                let num_end = self.index;

                // Tokenizer didn't move, so at end of string
                if num_start == num_end {
                    return Token::End;
                }

                let sub = match self.extract(num_start, num_end) {
                    Some(v) => v,
                    None => return Token::End,
                };
                let num: f64 = sub.parse().unwrap();
                return Token::Float(num);
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
        let input = "aldf eee, 1 1.0 1.00001 ::::, 100.123 123 ";
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.move_to_num();

        assert_eq!(tokenizer.get_current(), Some('1'));
    }

    #[test]
    fn get_token_float() {
        let input = "aldf eee, 1 1.0 1.00001 ::::, 100.123 123what 123 heyo 99";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(tokenizer.get_next_token(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token(), Token::Float(1.0));
        assert_eq!(tokenizer.get_next_token(), Token::Float(1.00001));
        assert_eq!(tokenizer.get_next_token(), Token::Float(100.123));
        assert_eq!(tokenizer.get_next_token(), Token::Float(123.0));
        assert_eq!(tokenizer.get_next_token(), Token::Float(123.0));
        assert_eq!(tokenizer.get_next_token(), Token::Float(99.0));
        assert_eq!(tokenizer.get_next_token(), Token::End);
    }

    #[test]
    fn get_token_comment() {
        let input = "# comment here \n # what more comment";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            tokenizer.get_next_token(),
            Token::Comment("comment here".to_string())
        );
        assert_eq!(
            tokenizer.get_next_token(),
            Token::Comment("what more comment".to_string())
        );
    }

    #[test]
    fn get_token_mttlib() {
        let input = "mttlib cube.mtl \n mttlib one more";
        let mut tokenizer = Tokenizer::new(input);

        assert_eq!(
            tokenizer.get_next_token(),
            Token::Mttlib("cube.mtl".to_string())
        );
        assert_eq!(
            tokenizer.get_next_token(),
            Token::Mttlib("one more".to_string())
        );
    }
}
