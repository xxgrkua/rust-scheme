use std::error;

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
    MissingCloseQuote,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected end of file")]
    UnexpectedEOF,

    #[error("invalid number: {0}")]
    InvalidNumber(String),

    #[error("missing opening parenthesis")]
    MissingOpenParenthesis,

    #[error("missing closing parenthesis")]
    MissingCLoseParenthesis,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0} is not implemented")]
    Unimplemented(String),

    #[error("{0}")]
    TokenError(TokenError),

    #[error("{0}")]
    ParseError(ParseError),
}

impl From<TokenError> for Error {
    fn from(error: TokenError) -> Self {
        Self::TokenError(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}
