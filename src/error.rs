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
}
