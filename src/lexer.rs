use std::fmt::Display;

use crate::error::TokenError;

type Result<T> = std::result::Result<T, TokenError>;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    Boolean(&'a str),
    Number(&'a str),
    Character(&'a str),
    String(&'a str),
    OpenParenthesis,
    CloseParenthesis,
    VectorOpen,
    ByteVectorOpen,
    Quote,
    Backquote,
    Comma,
    CommaAt,
    Dot,
}

impl<'a> Token<'a> {
    pub fn as_str(&self) -> &'a str {
        match *self {
            Self::Identifier(string) => string,
            Self::Boolean(string) => string,
            Self::Number(string) => string,
            Self::Character(string) => string,
            Self::String(string) => string,
            Self::OpenParenthesis => "(",
            Self::CloseParenthesis => ")",
            Self::VectorOpen => "#(",
            Self::ByteVectorOpen => "#u8(",
            Self::Quote => "'",
            Self::Backquote => "`",
            Self::Comma => ",",
            Self::CommaAt => ",@",
            Self::Dot => ".",
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

fn read_identifier<'a>(src: &'a str, index: &mut usize, length: usize) -> Result<Token<'a>> {
    Ok(Token::Dot)
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let token_list = vec![];
    let mut index = 0;
    let length = src.len();
    while index < length {
        match src.get(index..index + 1) {
            Some(string) => println!("{}", string),
            None => panic!(),
        }
        index += 1;
    }

    token_list
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let src = "(define x 4)";
        println!("{:?}", tokenize(src));
    }
}
