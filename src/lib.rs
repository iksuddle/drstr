/*!
A simple library for parsing human-readable duration strings into `std::time::Duration`.

## Usage

This library only provides [`parse`]:

```rust
use durstr::parse;
use std::time::Duration;

let dur = parse("12 minutes, 21 seconds");
assert_eq!(dur, Ok(Duration::from_secs(741)));

let dur = parse("1hr 2min 3sec");
assert_eq!(dur, Ok(Duration::from_secs(3723)));
```

## Supported Units

| Unit        | Aliases                               |
|-------------|---------------------------------------|
| Millisecond | `ms`, `msec`/`msecs`, `milliseconds`  |
| Second      | `s`, `sec`/`secs`, `seconds`          |
| Minute      | `m`, `min`/`mins`, `minutes`          |
| Hour        | `h`, `hr`/`hrs`, `hours`              |
*/

use std::{borrow::Cow, iter::Peekable, str::CharIndices, time::Duration};

/// An error that can occur when parsing a duration string.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    /// An unexpected character was found.
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
    /// An unexpected unit was found.
    #[error("unexpected unit: {0}")]
    UnexpectedUnit(String),
    /// A unit was expected, but not found.
    #[error("expected a unit")]
    ExpectedUnit,
    /// A number was expected, but not found.
    #[error("expected a number")]
    ExpectedNumber,
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Number(u32),
    Unit(&'a str),
}

struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Scanner<'a> {
    fn new(source: &'a str) -> Self {
        Scanner {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    fn scan_tokens(mut self) -> Result<Vec<Token<'a>>, Error> {
        let mut tokens = vec![];

        while let Some(&(i, c)) = self.chars.peek() {
            match c {
                c if self.should_skip(c) => {
                    self.chars.next();
                }
                c if c.is_ascii_digit() => {
                    tokens.push(Token::Number(self.scan_number(i)));
                }
                c if c.is_ascii_alphabetic() => {
                    tokens.push(Token::Unit(self.scan_unit(i)));
                }
                unexpected => return Err(Error::UnexpectedChar(unexpected)),
            };
        }

        Ok(tokens)
    }

    fn should_skip(&self, c: char) -> bool {
        c.is_ascii_whitespace() || c == ','
    }

    fn scan_number(&mut self, start: usize) -> u32 {
        let mut end = start;
        while let Some((_, c)) = self.chars.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            end = self.chars.next().unwrap().0;
        }

        self.source[start..=end].parse().unwrap()
    }

    fn scan_unit(&mut self, start: usize) -> &'a str {
        let mut end = start;
        while let Some((_, c)) = self.chars.peek() {
            if !c.is_ascii_alphabetic() {
                break;
            }
            end = self.chars.next().unwrap().0;
        }

        &self.source[start..=end]
    }
}

#[derive(Default)]
pub struct ParserOptions {
    ignore_case: bool,
}

#[derive(Default)]
pub struct Parser {
    options: ParserOptions,
}

impl Parser {
    pub fn new(options: ParserOptions) -> Self {
        Parser { options }
    }

    pub fn parse(&self, input: &str) -> Result<Duration, Error> {
        let tokens = Scanner::new(input).scan_tokens()?;
        self.parse_tokens(tokens)
    }

    fn parse_tokens(&self, tokens: Vec<Token>) -> Result<Duration, Error> {
        let mut tokens = tokens.into_iter();
        let mut dur = Duration::ZERO;

        while let Some(token) = tokens.next() {
            let num = match token {
                Token::Number(n) => n,
                Token::Unit(_) => return Err(Error::ExpectedNumber),
            };

            let unit = match tokens.next() {
                Some(Token::Unit(u)) => u,
                _ => return Err(Error::ExpectedUnit),
            };

            dur += num * self.get_unit_duration(unit)?;
        }

        Ok(dur)
    }

    fn get_unit_duration(&self, unit: &str) -> Result<Duration, Error> {
        let unit = if self.options.ignore_case {
            Cow::Owned(unit.to_lowercase())
        } else {
            Cow::Borrowed(unit)
        };

        match unit.as_ref() {
            "ms" | "msec" | "msecs" | "milliseconds" => Ok(Duration::from_millis(1)),
            "s" | "sec" | "secs" | "seconds" => Ok(Duration::from_secs(1)),
            "m" | "min" | "mins" | "minutes" => Ok(Duration::from_secs(60)),
            "h" | "hr" | "hrs" | "hours" => Ok(Duration::from_secs(3600)),
            _ => Err(Error::UnexpectedUnit(unit.into_owned())),
        }
    }
}

/// Parses a string into a `Duration`, ignoring whitespaces and commas.
///
/// ## Supported Units
/// - `ms`, `msec`/`msecs`, `milliseconds`
/// - `s`, `sec`/`secs`, `seconds`
/// - `m`, `min`/`mins`, `minutes`
/// - `h`, `hr`/`hrs`, `hours`
///
/// ## Examples
/// ```
/// use durstr::parse;
/// use std::time::Duration;
///
/// let d = parse("2 minutes, 12 seconds").unwrap();
/// assert_eq!(d, Duration::from_secs(132));
/// ```
///
/// This function uses a default [`Parser`].
/// Construct a new [`Parser`] to customize behavior (e.g. case sensitivity).
pub fn parse(input: &str) -> Result<Duration, Error> {
    Parser::default().parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{Error, Parser, ParserOptions, Scanner, Token, parse};
    use std::time::Duration;

    #[test]
    fn test_scanner() {
        let scanner = Scanner::new("10 seconds");
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens, Ok(vec![Token::Number(10), Token::Unit("seconds")]));

        let scanner = Scanner::new("9hr1min");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Number(9),
                Token::Unit("hr"),
                Token::Number(1),
                Token::Unit("min"),
            ])
        );

        let scanner = Scanner::new("712635 days");
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens, Ok(vec![Token::Number(712635), Token::Unit("days")]));
    }

    #[test]
    fn test_parsing() {
        let d = parse("2 minutes, 12 seconds");
        assert_eq!(d, Ok(Duration::from_secs(120 + 12)));

        let d = parse("45 msecs");
        assert_eq!(d, Ok(Duration::from_millis(45)));

        let d = parse("21 minutes 12 seconds");
        assert_eq!(d, Ok(Duration::from_secs(1272)));

        let d = parse("1 hr 2 mins 3 secs");
        assert_eq!(d, Ok(Duration::from_secs(3723)));

        let d = parse("1h 2min 3s 62ms");
        assert_eq!(d, Ok(Duration::from_millis(3723062)));

        let d = parse("1h2min3s62ms");
        assert_eq!(d, Ok(Duration::from_millis(3723062)));

        let d = parse("2min 3s62ms");
        assert_eq!(d, Ok(Duration::from_millis(123062)));

        let d = parse("2min 1*2 sec");
        assert_eq!(d, Err(Error::UnexpectedChar('*')));

        let d = parse("2 min 1 r");
        assert_eq!(d, Err(Error::UnexpectedUnit("r".to_owned())));

        let d = parse("2.1 min");
        assert_eq!(d, Err(Error::UnexpectedChar('.')));

        let d = parse("1 2");
        assert_eq!(d, Err(Error::ExpectedUnit));

        let d = parse("1 s m");
        assert_eq!(d, Err(Error::ExpectedNumber));
    }

    #[test]
    fn test_parsing_case_sensitivity() {
        let parser = Parser::new(ParserOptions { ignore_case: false });

        let d = parser.parse("1 min 2 sec");
        assert_eq!(d, Ok(Duration::from_secs(62)));

        let d = parser.parse("1 Min 2 sec");
        assert_eq!(d, Err(Error::UnexpectedUnit("Min".to_owned())));

        let d = parser.parse("1 min 2 seC");
        assert_eq!(d, Err(Error::UnexpectedUnit("seC".to_owned())));

        let parser = Parser::new(ParserOptions { ignore_case: true });

        let d = parser.parse("1 min 2 sec");
        assert_eq!(d, Ok(Duration::from_secs(62)));

        let d = parser.parse("1 Min 2 sec");
        assert_eq!(d, Ok(Duration::from_secs(62)));

        let d = parser.parse("1 min 2 seC");
        assert_eq!(d, Ok(Duration::from_secs(62)));

        let d = parser.parse("1 MIN 2 SEC");
        assert_eq!(d, Ok(Duration::from_secs(62)));
    }
}
