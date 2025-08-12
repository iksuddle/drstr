pub mod error;
mod scanner;

use std::time::Duration;

use error::Error;
use scanner::{Scanner, Token};

fn get_unit_duration(unit: &str) -> Result<Duration, Error> {
    let u = unit.to_lowercase();
    match u.as_str() {
        "ms" | "msec" | "milliseconds" => Ok(Duration::from_millis(1)),
        "s" | "sec" | "seconds" => Ok(Duration::from_secs(1)),
        "m" | "min" | "minutes" => Ok(Duration::from_secs(60)),
        "h" | "hr" | "hours" => Ok(Duration::from_secs(3600)),
        _ => Err(Error::UnexpectedUnit(u)),
    }
}

pub fn parse(input: &str) -> Result<Duration, Error> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    parse_tokens(tokens)
}

fn parse_tokens(tokens: Vec<Token>) -> Result<Duration, Error> {
    let mut tokens = tokens.into_iter();

    let mut dur = Duration::ZERO;
    loop {
        let num = match tokens.next() {
            None => break,
            Some(Token::Unit(_)) => return Err(Error::ExpectedNumber),
            Some(Token::Number(n)) => n,
        };

        let unit = match tokens.next() {
            Some(Token::Unit(u)) => u,
            _ => return Err(Error::ExpectedUnit),
        };

        dur += num * get_unit_duration(&unit)?;
    }

    Ok(dur)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{error::Error, parse};

    #[test]
    fn test_scanner() {
        // let mut scanner = Scanner::new("10 seconds");
        // let tokens = scanner.scan_tokens();
        // assert_eq!(
        //     tokens,
        //     Ok(vec![Token::Number(10), Token::Unit("seconds".to_string())])
        // );
        //
        // let mut scanner = Scanner::new("9hr1min");
        // let tokens = scanner.scan_tokens();
        // assert_eq!(
        //     tokens,
        //     Ok(vec![
        //         Token::Number(9),
        //         Token::Unit("hr".to_string()),
        //         Token::Number(1),
        //         Token::Unit("min".to_string()),
        //     ])
        // );
        //
        // let mut scanner = Scanner::new("712635 days");
        // let tokens = scanner.scan_tokens();
        // assert_eq!(
        //     tokens,
        //     Ok(vec![Token::Number(712635), Token::Unit("days".to_string())])
        // );
    }

    #[test]
    fn test_parsing() {
        let d = parse("2 minutes, 12 seconds");
        assert_eq!(d, Ok(Duration::from_secs(120 + 12)));

        let d = parse("45msec");
        assert_eq!(d, Ok(Duration::from_millis(45)));

        let d = parse("21 minutes 12 seconds");
        assert_eq!(d, Ok(Duration::from_secs(1272)));

        let d = parse("1 hr 2 min 3 sec");
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
}
