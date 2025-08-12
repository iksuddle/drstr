#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("unexpected unit: {0}")]
    UnexpectedUnit(String),
    #[error("expected a unit")]
    ExpectedUnit,
    #[error("expected a number")]
    ExpectedNumber,
}
