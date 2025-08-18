/*!
A simple library for parsing human-readable duration strings into `std::time::Duration`.

## Usage

This library provides a [`parse`] function for quick and easy parsing, and a [`Parser`]
struct for more control over parsing behavior.

### The `parse` function

The [`parse`] function is a convenience wrapper around a default [`Parser`].

```rust
use durstr::parse;
use std::time::Duration;

let dur = parse("12 minutes, 21 seconds");
assert_eq!(dur, Ok(Duration::from_secs(741)));

let dur = parse("1hr 2min 3sec");
assert_eq!(dur, Ok(Duration::from_secs(3723)));
```

### The `Parser` struct

For more control, you can use the [`Parser`] struct directly. For example, to parse with case-insensitivity:

```rust
use durstr::{Parser, ParserOptions};
use std::time::Duration;

let parser = Parser::new(ParserOptions { ignore_case: true });
let dur = parser.parse("1 MINUTE, 2 SECONDS");
assert_eq!(dur, Ok(Duration::from_secs(62)));
```

## Supported Units

| Unit        | Aliases                            |
|-------------|------------------------------------|
| Millisecond | `ms`, `msec(s)`, `millisecond(s)`  |
| Second      | `s`, `sec(s)`, `second(s)`         |
| Minute      | `m`, `min(s)`, `minute(s)`         |
| Hour        | `h`, `hr(s)`, `hour(s)`            |
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

/// Options to customize the behavior of a [`Parser`].
///
/// This struct allows for more control over how duration strings are
/// interpreted. (e.g. enabling case-insensitivity)
#[derive(Default)]
pub struct ParserOptions {
    pub ignore_case: bool,
}

/// A configurable parser for duration strings.
///
/// Use this when you need to configure the parsing logic. Otherwise, the
/// top-level [`parse`] function is likely sufficient.
#[derive(Default)]
pub struct Parser {
    options: ParserOptions,
}

impl Parser {
    /// Create a new [`Parser`] with provided [`ParserOptions`]
    pub fn new(options: ParserOptions) -> Self {
        Parser { options }
    }

    /// Parses a string into a `Duration`, ignoring whitespaces and commas.
    ///
    /// ## Supported Units
    /// - `ms`, `msec(s)`, `millisecond(s)`
    /// - `s`, `sec(s)`, `second(s)`
    /// - `m`, `min(s)`, `minute(s)`
    /// - `h`, `hr(s)`, `hour(s)`
    ///
    /// ## Examples
    /// ```
    /// use durstr::{Parser, ParserOptions};
    /// use std::time::Duration;
    ///
    /// let parser = Parser::new(ParserOptions { ignore_case: true });
    /// let dur = parser.parse("1 MINUTE, 2 SECONDS");
    /// assert_eq!(dur, Ok(Duration::from_secs(62)));
    /// ```
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
            "h" | "hr" | "hrs" | "hour" | "hours" => Ok(Duration::from_secs(3600)),
            "m" | "min" | "mins" | "minute" | "minutes" => Ok(Duration::from_secs(60)),
            "s" | "sec" | "secs" | "second" | "seconds" => Ok(Duration::from_secs(1)),
            "ms" | "msec" | "msecs" | "millisecond" | "milliseconds" => {
                Ok(Duration::from_millis(1))
            }
            _ => Err(Error::UnexpectedUnit(unit.into_owned())),
        }
    }
}

/// Parses a duration string into a `std::time::Duration`.
///
/// This function provides a quick and easy way to parse common duration
/// formats. It is a convenience wrapper around a default [`Parser`], which is
/// case-sensitive and ignores whitespace and commas.
///
/// For more control over parsing behavior, such as enabling case-insensitivity,
/// construct a [`Parser`] with custom [`ParserOptions`].
///
/// ## Examples
/// ```
/// use durstr::parse;
/// use std::time::Duration;
///
/// let dur = parse("12 minutes, 21 seconds");
/// assert_eq!(dur, Ok(Duration::from_secs(741)));
///
/// let dur = parse("1hr 2min 3sec");
/// assert_eq!(dur, Ok(Duration::from_secs(3723)));
///
/// // By default, parsing is case-sensitive.
/// let dur = parse("1 MINUTE");
/// assert!(dur.is_err());
/// ```
pub fn parse(input: &str) -> Result<Duration, Error> {
    Parser::default().parse(input)
}

#[cfg(test)]
mod tests {
    use crate::{Scanner, Token};

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
}
