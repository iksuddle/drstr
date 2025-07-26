use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(u64),
    Unit(String),
    Eof,
}

struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Scanner {
            chars: source.chars().peekable(),
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(c) = self.chars.peek() {
            tokens.push(match c {
                ' ' | '\n' | '\t' => {
                    self.chars.next();
                    continue;
                }
                '0'..='9' => Token::Number(self.scan_number()),
                _ => Token::Unit(self.scan_unit()),
            });
        }

        tokens.push(Token::Eof);

        tokens
    }

    fn scan_number(&mut self) -> u64 {
        let mut literal = String::new();

        while let Some(c) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            literal.push(self.chars.next().unwrap());
        }

        literal.parse().unwrap() // todo: don't unwrap
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

#[cfg(test)]
mod tests {
    use crate::{Scanner, Token};

    #[test]
    fn test_scanner() {
        let mut scanner = Scanner::new("10 seconds");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Token::Number(10),
                Token::Unit("seconds".to_owned()),
                Token::Eof
            ]
        );

        let mut scanner = Scanner::new("9hr1min");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Token::Number(9),
                Token::Unit("hr".to_owned()),
                Token::Number(1),
                Token::Unit("min".to_owned()),
                Token::Eof
            ]
        );

        let mut scanner = Scanner::new("712635 days");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Token::Number(712635),
                Token::Unit("days".to_owned()),
                Token::Eof
            ]
        );
    }
}
