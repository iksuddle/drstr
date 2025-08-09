use std::{iter::Peekable, num::ParseIntError, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(u32),
    Unit(String),
}

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            chars: source.chars().peekable(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = vec![];

        while let Some(c) = self.chars.peek() {
            tokens.push(match c {
                ' ' | '\n' | '\t' | ',' => {
                    self.chars.next();
                    continue;
                }
                '0'..='9' => Token::Number(self.scan_number()?),
                'a'..='z' | 'A'..='Z' => Token::Unit(self.scan_unit()),
                x => return Err(format!("unexpected character: {}", x)),
                // todo: nice error message
            });
        }

        Ok(tokens)
    }

    fn scan_number(&mut self) -> Result<u32, String> {
        let mut literal = String::new();

        while let Some(c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            literal.push(self.chars.next().unwrap());
        }

        literal.parse().map_err(|e: ParseIntError| e.to_string())
    }

    fn scan_unit(&mut self) -> String {
        let mut literal = String::new();

        while let Some(c) = self.chars.peek() {
            if !c.is_ascii_alphabetic() {
                break; // todo: include location in error msg
            }
            literal.push(self.chars.next().unwrap());
        }

        literal
    }
}
