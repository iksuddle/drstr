use std::time::Duration;

use scanner::{Scanner, Token};

mod scanner;

fn get_unit_duration(unit: &str) -> Duration {
    match unit.to_lowercase().as_str() {
        "ms" | "msec" | "milliseconds" => Duration::from_millis(1),
        "s" | "sec" | "seconds" => Duration::from_secs(1),
        "m" | "min" | "minutes" => Duration::from_secs(60),
        "h" | "hr" | "hours" => Duration::from_secs(3600),
        _ => todo!(),
    }
}

pub fn parse(input: &str) -> Result<Duration, String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();
    let mut tokens = tokens.iter();

    let mut dur = Duration::from_secs(0);

    while let Some(tok) = tokens.next() {
        match tok {
            // always expect a number before a unit
            Token::Unit(unit) => return Err(format!("unexpected unit: {}", unit)),
            Token::Number(num) => {
                if let Some(Token::Unit(unit)) = tokens.next() {
                    let added_duration = *num * get_unit_duration(unit);
                    dur += added_duration;
                } else {
                    return Err("expected unit after number".to_string());
                }
            }
            Token::Eof => break,
        }
    }

    Ok(dur)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        parse,
        scanner::{Scanner, Token},
    };

    #[test]
    fn test_scanner() {
        let mut scanner = Scanner::new("10 seconds");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Token::Number(10),
                Token::Unit("seconds".to_string()),
                Token::Eof
            ]
        );

        let mut scanner = Scanner::new("9hr1min");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Token::Number(9),
                Token::Unit("hr".to_string()),
                Token::Number(1),
                Token::Unit("min".to_string()),
                Token::Eof
            ]
        );

        let mut scanner = Scanner::new("712635 days");
        let tokens = scanner.scan_tokens();
        assert_eq!(
            tokens,
            vec![
                Token::Number(712635),
                Token::Unit("days".to_string()),
                Token::Eof
            ]
        );
    }

    #[test]
    fn test_parsing() {
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

        let d = parse("2min s");
        assert_eq!(d, Err("unexpected unit: s".to_string()));
    }
}
