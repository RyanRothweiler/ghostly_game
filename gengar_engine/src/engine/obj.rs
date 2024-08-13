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
    Float(f64),
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

    pub fn get_current(&self) -> Option<char> {
        if self.index >= self.data.len() {
            return None;
        } else {
            return Some(self.data[self.index]);
        }
    }

    pub fn get_token(&mut self) -> Token {
        self.move_to_num();
        let num_start = self.index;

        self.move_until(|c| c.is_alphabetic() || c.is_whitespace());

        let num_end = self.index;

        // Tokenizer didn't move, so at end of string
        if num_start == num_end {
            return Token::End;
        }

        let sub = &self.data[num_start..num_end];
        let sub: Vec<char> = sub.iter().cloned().collect();
        let sub: String = sub.into_iter().collect();

        let num: f64 = sub.parse().unwrap();
        return Token::Float(num);
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

        assert_eq!(tokenizer.get_token(), Token::Float(1.0));
        assert_eq!(tokenizer.get_token(), Token::Float(1.0));
        assert_eq!(tokenizer.get_token(), Token::Float(1.00001));
        assert_eq!(tokenizer.get_token(), Token::Float(100.123));
        assert_eq!(tokenizer.get_token(), Token::Float(123.0));
        assert_eq!(tokenizer.get_token(), Token::Float(123.0));
        assert_eq!(tokenizer.get_token(), Token::Float(99.0));
        assert_eq!(tokenizer.get_token(), Token::End);
    }
}
