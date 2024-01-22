use std::{
    fmt::Display,
    ops::{Deref, Index, Range, RangeInclusive},
};

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
const COMMENT: &'static str = ";";

include!(concat!(env!("OUT_DIR"), "/tokens.rs"));

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    Boolean(&'a str),
    Number(&'a str),
    Character(&'a str),
    String(&'a str),
    Comment(&'a str),
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
            Self::Comment(string) => string,
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

#[derive(Debug, Clone, Copy)]
struct Buffer<'a> {
    src: &'a str,
    length: usize,
}

impl<'a> Buffer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src,
            length: src.len(),
        }
    }

    fn get(&self, index: usize) -> (&'a str, usize, usize) {
        let start = index;
        let mut end = index + 1;
        while end <= self.length {
            match self.src.get(start..end) {
                Some(character) => {
                    return (character, start, end);
                }
                None => {
                    end += 1;
                }
            }
        }
        ("", start, end)
    }
}

impl<'a> Deref for Buffer<'a> {
    type Target = str;

    fn deref(&self) -> &'a Self::Target {
        self.src
    }
}

impl<'a> Index<Range<usize>> for Buffer<'a> {
    type Output = str;

    fn index(&self, index: Range<usize>) -> &'a Self::Output {
        &self.src[index]
    }
}

impl<'a> Index<RangeInclusive<usize>> for Buffer<'a> {
    type Output = str;

    fn index(&self, index: RangeInclusive<usize>) -> &'a Self::Output {
        &self.src[index]
    }
}

fn read_comment<'a>(
    buffer: &Buffer<'a>,
    start_index: usize,
    mut index: usize,
    length: usize,
) -> Result<(Token<'a>, usize)> {
    while index < length {
        let (character, _, end) = buffer.get(index);
        if NEWLINE.contains(character) {
            return Ok((Token::Comment(&buffer.src[start_index..index]), end));
        } else {
            index = end;
        }
    }
    Ok((Token::Comment(&buffer.src[start_index..index]), index))
}

fn read_identifier<'a>(
    buffer: &Buffer<'a>,
    start_index: usize,
    mut index: usize,
    length: usize,
) -> Result<(Token<'a>, usize)> {
    while index < length {
        match buffer.get(index) {
            (character, _, end) if SUBSEQUENT.contains(character) => {
                index = end;
            }
            (character, _, end) if DELIMITER.contains(character) => {
                return Ok((Token::Identifier(&buffer.src[start_index..index]), end));
            }
            (_, _, end) => {
                return Err(TokenError::InvalidIdentifier(
                    buffer.src[start_index..end].to_string(),
                ));
            }
        }
    }
    Ok((Token::Identifier(&buffer.src[start_index..index]), index))
}

fn read_number<'a>(
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
            return Ok((Token::Number(&src[start_index..index]), index));
        } else {
            return Err(TokenError::InvalidIdentifier(
                src[start_index..index + 1].to_string(),
            ));
        }
    }
    Ok((Token::Number(&src[start_index..index]), index))
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let buffer = Buffer::new(src);
    let mut token_list = vec![];
    let mut index = 0;
    let mut token: Token;
    let length = src.len();
    while index < buffer.length {
        match buffer.get(index) {
            ("", _, _) => {
                return Ok(token_list);
            }
            (OPEN_PARENTHESIS, _, end) => {
                token_list.push(Token::OpenParenthesis);
                index = end;
            }
            (CLOSE_PARENTHESIS, _, end) => {
                token_list.push(Token::CloseParenthesis);
                index = end;
            }
            (QUOTE, _, end) => {
                token_list.push(Token::Quote);
                index = end;
            }
            (BACKQUOTE, _, end) => {
                token_list.push(Token::Backquote);
                index = end;
            }
            (COMMA, _, end) => {
                if buffer.get(end) == ("@", end, end + 1) {
                    token_list.push(Token::CommaAt);
                    index = end + 1;
                } else {
                    token_list.push(Token::Comma);
                    index = end;
                }
            }
            (DOT, start, end) => {
                let (character2, _, end2) = buffer.get(end);
                if DELIMITER.contains(character2) {
                    token_list.push(Token::Dot);
                    index = end;
                } else {
                    let (character3, _, end3) = buffer.get(end2);
                    let (character4, _, end4) = buffer.get(end3);
                    if (character2 == DOT) && (character3 == DOT) && DELIMITER.contains(character4)
                    {
                        token_list.push(Token::Identifier("..."));
                        index = end4;
                    } else {
                        return Err(TokenError::InvalidIdentifier(
                            buffer.src[start..end2].to_string(),
                        ));
                    }
                }
            }
            (COMMENT, start, end) => {
                (token, index) = read_comment(&buffer, start, end, length)?;
                token_list.push(token);
            }
            ("#", start, end) => {
                let (character2, _, end2) = buffer.get(end);
                match character2 {
                    "t" | "f" => {}
                    _ => return Err(TokenError::InvalidConstant(buffer[start..end2].to_string())),
                }
                return Err(TokenError::InvalidCharacter("#".to_string()));
            }
            ("+", start, end) | ("-", start, end) => {
                let (character, _, second_end) = buffer.get(end);
                if DELIMITER.contains(character) {
                    token_list.push(Token::Identifier(&buffer.src[start..end]));
                    index = second_end;
                } else {
                    return Err(TokenError::InvalidIdentifier(
                        buffer.src[start..end].to_string(),
                    ));
                }
            }
            (character, _, end) if WHITESPACE.contains(character) => {
                index = end;
            }
            (character, start, end) if INITIAL.contains(character) => {
                (token, index) = read_identifier(&buffer, start, end, length)?;
                token_list.push(token);
            }
            (character, start, end) if DIGIT.contains(character) => {
                (token, index) = read_number(src, start, end, length)?;
                token_list.push(token);
            }
            (character, start, end) => {
                println!("{} {} {}", character, start, end);
                index = end;
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
        println!("{:?}", tokenize("; this is a comment"));
        println!("{:?}", tokenize("(define a,b 4)"));
        println!("{:?}", tokenize("`(1 a ,@(maps s d))"));
        let src = "asdfvasgdsa df asdfvgwae sdfasdfva 我是大肥猪";
        let buffer = Buffer::new(src);
        let mut i = 0;
        while i < src.len() {
            let (character, start, end) = buffer.get(i);
            println!("{} {} {}", character, start, end);
            i = end;
        }
    }
}
