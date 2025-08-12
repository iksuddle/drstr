use std::{iter::Peekable, str::Chars};

use crate::error::Error;

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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];

        while let Some(c) = self.chars.peek() {
            tokens.push(match c {
                ' ' | '\n' | '\t' | ',' => {
                    self.chars.next();
                    continue;
                }
                '0'..='9' => Token::Number(self.scan_number()),
                'a'..='z' | 'A'..='Z' => Token::Unit(self.scan_unit()),
                x => return Err(Error::UnexpectedChar(*x)),
            });
        }

        Ok(tokens)
    }

    fn scan_number(&mut self) -> u32 {
        let mut literal = String::new();

        while let Some(c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            literal.push(self.chars.next().unwrap());
        }

        literal.parse().unwrap()
    }

    fn scan_unit(&mut self) -> String {
        let mut literal = String::new();

        while let Some(c) = self.chars.peek() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            literal.push(self.chars.next().unwrap());
        }

        literal
    }
}
