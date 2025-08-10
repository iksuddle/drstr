use std::{iter::Peekable, num::ParseIntError, str::Chars};

#[derive(thiserror::Error, Debug)]
pub enum ScannerError {
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("failed to parse int")]
    ParseIntError(#[from] ParseIntError),
}

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

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut tokens = vec![];

        while let Some(c) = self.chars.peek() {
            tokens.push(match c {
                ' ' | '\n' | '\t' | ',' => {
                    self.chars.next();
                    continue;
                }
                '0'..='9' => Token::Number(self.scan_number()?),
                'a'..='z' | 'A'..='Z' => Token::Unit(self.scan_unit()),
                x => return Err(ScannerError::UnexpectedChar(*x)),
            });
        }

        Ok(tokens)
    }

    fn scan_number(&mut self) -> Result<u32, ScannerError> {
        let mut literal = String::new();

        while let Some(c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            literal.push(self.chars.next().unwrap());
        }

        Ok(literal.parse()?)
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
