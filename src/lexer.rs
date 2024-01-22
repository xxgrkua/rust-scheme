use std::fmt::Display;

use crate::error::TokenError;

type Result<T> = std::result::Result<T, TokenError>;

const OPEN_PARENTHESIS: &'static str = "(";
const CLOSE_PARENTHESIS: &'static str = ")";
const VECTOR_OPEN: &'static str = "#(";
const BYTE_VECTOR_OPEN: &'static str = "#u8(";
const QUOTE: &'static str = "'";
const BACKQUOTE: &'static str = "`";
const COMMA: &'static str = ",";
const COMMA_AT: &'static str = ",@";
const DOT: &'static str = ".";

include!(concat!(env!("OUT_DIR"), "/tokens.rs"));

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
            Self::OpenParenthesis => OPEN_PARENTHESIS,
            Self::CloseParenthesis => CLOSE_PARENTHESIS,
            Self::VectorOpen => VECTOR_OPEN,
            Self::ByteVectorOpen => BYTE_VECTOR_OPEN,
            Self::Quote => QUOTE,
            Self::Backquote => BACKQUOTE,
            Self::Comma => COMMA,
            Self::CommaAt => COMMA_AT,
            Self::Dot => DOT,
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

fn read_identifier<'a>(
    src: &'a str,
    start_index: usize,
    mut index: usize,
    length: usize,
) -> Result<(Token<'a>, usize)> {
    while index < length {
        let character = &src[index..index + 1];
        if SUBSEQUENT.contains(character) {
            index += 1;
        } else if DELIMITER.contains(character) {
            return Ok((Token::Identifier(&src[start_index..index]), index));
        } else {
            return Err(TokenError::InvalidCharacter(character.to_string()));
        }
    }
    Ok((Token::Identifier(&src[start_index..index]), index))
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut token_list = vec![];
    let mut index = 0;
    let mut token: Token;
    let length = src.len();
    while index < length {
        let character = &src[index..index + 1];
        match character {
            OPEN_PARENTHESIS => {
                token_list.push(Token::OpenParenthesis);
                index += 1;
            }
            CLOSE_PARENTHESIS => {
                token_list.push(Token::CloseParenthesis);
                index += 1;
            }
            VECTOR_OPEN => {
                token_list.push(Token::VectorOpen);
                index += 2;
            }
            BYTE_VECTOR_OPEN => {
                token_list.push(Token::ByteVectorOpen);
                index += 3;
            }
            QUOTE => {
                token_list.push(Token::Quote);
                index += 1;
            }
            BACKQUOTE => {
                token_list.push(Token::Backquote);
                index += 1;
            }
            COMMA => {
                token_list.push(Token::Comma);
                index += 1;
            }
            COMMA_AT => {
                token_list.push(Token::CommaAt);
                index += 2;
            }
            DOT => {
                token_list.push(Token::Dot);
                index += 1;
            }
            character => {
                if WHITESPACE.contains(character) {
                    index += 1;
                } else if INITIAL.contains(character) {
                    (token, index) = read_identifier(src, index, index + 1, length)?;
                    token_list.push(token);
                } else if PECULIAR_IDENTIFIER.contains(character) {
                    if (index + 2 == length) || DELIMITER.contains(&src[index + 1..index + 2]) {
                        token_list.push(Token::Identifier(character));
                        index += 1;
                    }
                } else if (index + 3 <= length)
                    && PECULIAR_IDENTIFIER.contains(&src[index..index + 3])
                {
                    if (index + 4 == length) || DELIMITER.contains(&src[index + 3..index + 4]) {
                        token_list.push(Token::Identifier(character));
                        index += 3;
                    }
                } else {
                    println!("{}", character);
                    index += 1;
                }
            }
        }
    }

    Ok(token_list)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let src = "(define x 4)";
        println!("{:?}", tokenize(src));
        println!("{:?}", tokenize("(+ 1 2)"));
    }
}
