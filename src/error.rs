use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("invalid character: {0}")]
    InvalidCharacter(String),

    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[error("invalid constant literal: {0}")]
    InvalidConstant(String),

    #[error("invalid number literal: {0}")]
    InvalidNumber(String),

    #[error("invalid escape in string: {0}")]
    InvalidStringEscape(String),

    #[error("string missing closing quote")]
    MissingClosingQuote,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected end of file")]
    UnexpectedEOF,
}
