use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("invalid character: {0}")]
    InvalidCharacter(String),

    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),
}
