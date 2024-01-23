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
const STRING: &'static str = "\"";

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
        ("", start, end.min(self.length))
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

fn read_until_delimiter<'a>(buffer: &Buffer<'a>, start_index: usize) -> (&'a str, usize) {
    let mut index = start_index + 1;
    while index < buffer.length {
        let (character, _, end) = buffer.get(index);
        if DELIMITER.contains(character) {
            return (&buffer.src[start_index..index], end);
        } else {
            index = end;
        }
    }
    (&buffer.src[start_index..index], index)
}

fn read_comment<'a>(
    buffer: &Buffer<'a>,
    start_index: usize,
    mut index: usize,
) -> Result<(Token<'a>, usize)> {
    while index < buffer.length {
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
) -> Result<(Token<'a>, usize)> {
    while index < buffer.length {
        match buffer.get(index) {
            (character, _, end) if SUBSEQUENT.contains(character) => {
                index = end;
            }
            (character, _, end) if DELIMITER.contains(character) => {
                return Ok((Token::Identifier(&buffer.src[start_index..index]), end));
            }
            (_, _, end) => {
                return Err(TokenError::InvalidIdentifier(format!(
                    "{}{}",
                    &buffer.src[start_index..end],
                    read_until_delimiter(buffer, end).0
                )));
            }
        }
    }
    Ok((Token::Identifier(&buffer.src[start_index..index]), index))
}

fn read_number<'a>(
    buffer: &Buffer<'a>,
    start_index: usize,
    mut index: usize,
) -> Result<(Token<'a>, usize)> {
    while index < buffer.length {
        match buffer.get(index) {
            (character, _, end) if DIGIT.contains(character) => {
                index = end;
            }
            (".", _, end) => {
                index = end;
            }
            (character, _, end) if DELIMITER.contains(character) => {
                return Ok((Token::Number(&buffer.src[start_index..index]), end));
            }
            (_, _, end) => {
                return Err(TokenError::InvalidNumber(format!(
                    "{}{}",
                    &buffer.src[start_index..end],
                    read_until_delimiter(buffer, end).0
                )));
            }
        }
    }
    Ok((Token::Number(&buffer.src[start_index..index]), index))
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let buffer = Buffer::new(src);
    let mut token_list = vec![];
    let mut index = 0;
    let mut token: Token;
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
                        return Err(TokenError::InvalidIdentifier(format!(
                            "{}{}",
                            &buffer.src[start..end4],
                            read_until_delimiter(&buffer, end4).0
                        )));
                    }
                }
            }
            (COMMENT, start, end) => {
                (token, index) = read_comment(&buffer, start, end)?;
                token_list.push(token);
            }
            ("#", start, end) => {
                let (character2, _, end2) = buffer.get(end);
                match character2 {
                    "t" | "f" => {}
                    _ => {
                        return Err(TokenError::InvalidConstant(format!(
                            "{}{}",
                            &buffer.src[start..end2],
                            read_until_delimiter(&buffer, end2).0
                        )))
                    }
                }
                return Err(TokenError::InvalidCharacter("#".to_string()));
            }
            ("+", start, end) | ("-", start, end) => match buffer.get(end) {
                (character2, _, end2) if DELIMITER.contains(character2) => {
                    token_list.push(Token::Identifier(&buffer.src[start..end]));
                    index = end2;
                }
                (character2, _, end2) if DIGIT.contains(character2) => {
                    (token, index) = read_number(&buffer, start, end2)?;
                    token_list.push(token);
                }
                (_, _, end2) => {
                    return Err(TokenError::InvalidIdentifier(format!(
                        "{}{}",
                        &buffer.src[start..end2],
                        read_until_delimiter(&buffer, end2).0
                    )));
                }
            },
            (character, _, end) if WHITESPACE.contains(character) => {
                index = end;
            }
            (character, start, end) if INITIAL.contains(character) => {
                (token, index) = read_identifier(&buffer, start, end)?;
                token_list.push(token);
            }
            (character, start, end) if DIGIT.contains(character) => {
                (token, index) = read_number(&buffer, start, end)?;
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
        println!("{:?}", tokenize("(... 1 2 3)"));
        println!("{:?}", tokenize("(.... 1 2 3)"));
        println!("{:?}", tokenize(".."));
        println!("{:?}", tokenize("(+ +2 3)"));
        println!("{:?}", tokenize("(+ ++2 3)"));
        println!("{:?}", tokenize("(+ #asda #aaa)"));
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
