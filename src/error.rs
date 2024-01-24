use std::{error, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TokenError {
    #[error("invalid character: {0}")]
    InvalidCharacter(String),

    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[error("invalid constant literal: {0}")]
    InvalidConstant(String),

    #[error("invalid number literal: {0}")]
    InvalidNumber(String),

    #[error("string missing closing quote")]
    MissingCloseQuote,
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("unexpected end of file")]
    EOF,

    #[error("invalid number: {0}")]
    InvalidNumber(String),

    #[error("missing opening parenthesis")]
    MissingOpenParenthesis,

    #[error("missing closing parenthesis")]
    MissingCLoseParenthesis,

    #[error("invalid escape in string: {0}")]
    InvalidCharacterEscape(String),

    #[error("only one object is allowed after a dot")]
    TooMoreObjects,
}

#[derive(Debug, Error, PartialEq)]
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
