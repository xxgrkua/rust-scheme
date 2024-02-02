use std::{fmt::Display, rc::Rc};

use crate::{
    error::ParseError,
    lexer::{Token, TokenBuffer},
    number::Number,
};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'a> {
    content: Link<'a>,
}

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl<'a> Expression<'a> {
    pub fn car(&self) -> Self {
        if let Link(Some(expression)) = &self.content {
            if let ExpressionContent::PairLink(pair) = expression.as_ref() {
                return Self {
                    content: pair.car.clone(),
                };
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Link<'a>(Option<Rc<ExpressionContent<'a>>>);

impl<'a> Display for Link<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self(Some(expression)) = self {
            write!(f, "{}", expression)
        } else {
            write!(f, "()")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ExpressionContent<'a> {
    Number(Number),
    String(String),
    Boolean(bool),
    Symbol(&'a str),
    PairLink(Pair<'a>),
    VectorLink(Vec<Link<'a>>),
}

impl<'a> Display for ExpressionContent<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "\"{}\"", string),
            Self::Boolean(boolean) => {
                if *boolean {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::PairLink(pair) => write!(f, "{}", pair),
            Self::VectorLink(vector) => {
                write!(f, "#(")?;
                for (index, link) in vector.iter().enumerate() {
                    write!(f, "{}", link)?;
                    if index != vector.len() - 1 {
                        write!(f, " ")?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pair<'a> {
    car: Link<'a>,
    cdr: Link<'a>,
}

impl<'a> Display for Pair<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", &self.car)?;
        let mut cdr = &self.cdr;
        loop {
            if let Some(expression) = &cdr.0 {
                if let ExpressionContent::PairLink(pair) = expression.as_ref() {
                    write!(f, " {}", &pair.car)?;
                    cdr = &pair.cdr;
                } else {
                    write!(f, " . {}", cdr)?;
                    break;
                };
                // the above semi-colon is important,
                // or the value will be dropped while the it is still borrowed
                // ref: https://smallcultfollowing.com/babysteps/blog/2023/03/15/temporary-lifetimes/
                // ref: https://doc.rust-lang.org/nightly/reference/destructors.html#drop-scopes
            } else {
                break;
            }
        }
        write!(f, ")")
    }
}

pub fn parse<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression<'a>> {
    if buffer.is_empty() {
        return Err(ParseError::EOF);
    }
    match *buffer.pop() {
        Token::Identifier(identifier) => Ok(Expression {
            content: Link(Some(Rc::new(ExpressionContent::Symbol(identifier)))),
        }),
        Token::Boolean(value) => match value {
            "#t" | "#true" => Ok(Expression {
                content: Link(Some(Rc::new(ExpressionContent::Boolean(true)))),
            }),
            "#f" | "#false" => Ok(Expression {
                content: Link(Some(Rc::new(ExpressionContent::Boolean(false)))),
            }),
            _ => unreachable!(),
        },
        Token::Number(number) => Ok(Expression {
            content: Link(Some(Rc::new(ExpressionContent::Number(Number::try_from(
                number,
            )?)))),
        }),
        Token::String(string) => {
            let mut result = String::new();
            let mut start_escape = false;
            let mut start_hex = false;
            let mut hex_buffer = String::new();
            for character in string.chars() {
                match character {
                    '\\' => {
                        if start_escape {
                            result.push('\\');
                            start_escape = false;
                        } else {
                            start_escape = true;
                        }
                    }
                    '"' => {
                        if start_escape {
                            result.push('"');
                            start_escape = false;
                        } else {
                            // remove the first and last quote
                            continue;
                        }
                    }
                    'n' => {
                        if start_escape {
                            result.push('\n');
                            start_escape = false;
                        } else {
                            result.push('n');
                        }
                    }
                    'r' => {
                        if start_escape {
                            result.push('\r');
                            start_escape = false;
                        } else {
                            result.push('r');
                        }
                    }
                    't' => {
                        if start_escape {
                            result.push('\t');
                            start_escape = false;
                        } else {
                            result.push('t');
                        }
                    }
                    'b' => {
                        if start_escape {
                            result.push('\x08');
                            start_escape = false;
                        } else {
                            result.push('b');
                        }
                    }
                    'a' => {
                        if start_escape {
                            result.push('\x07');
                            start_escape = false;
                        } else {
                            result.push('a');
                        }
                    }
                    'x' => {
                        if start_escape {
                            start_hex = true;
                        } else {
                            result.push('x');
                        }
                    }
                    ';' => {
                        if start_hex {
                            start_hex = false;
                            start_escape = false;
                            let hex = u32::from_str_radix(&hex_buffer, 16).map_err(|_| {
                                ParseError::InvalidCharacterEscape(format!("\\x{}", hex_buffer))
                            })?;
                            result.push(char::from_u32(hex).ok_or(
                                ParseError::InvalidCharacterEscape(format!("\\x{}", hex_buffer)),
                            )?);
                            hex_buffer.clear();
                        } else {
                            result.push(';');
                        }
                    }
                    c => {
                        if start_hex {
                            if c.is_digit(16) {
                                hex_buffer.push(c);
                            } else {
                                return Err(ParseError::InvalidCharacterEscape(format!(
                                    "\\x{}{}",
                                    hex_buffer, c
                                )));
                            }
                        } else if start_escape {
                            return Err(ParseError::InvalidCharacterEscape(format!("\\{}", c)));
                        } else {
                            result.push(c);
                        }
                    }
                }
            }
            Ok(Expression {
                content: Link(Some(Rc::new(ExpressionContent::String(result)))),
            })
        }
        Token::Comment(_) => parse(buffer),
        Token::OpenParenthesis => parse_pair(buffer),
        Token::CloseParenthesis => Err(ParseError::MissingOpenParenthesis),
        Token::VectorOpen => {
            // vector is self-evaluating in R7RS
            let vector = vec![];
            parse_vector(buffer, vector)
        }
        Token::ByteVectorOpen => unimplemented!(),
        Token::Quote => Ok(Expression {
            content: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link(Some(Rc::new(ExpressionContent::Symbol("quote")))),
                cdr: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link(None),
                })))),
            })))),
        }),
        Token::BackQuote => Ok(Expression {
            content: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link(Some(Rc::new(ExpressionContent::Symbol("quasiquote")))),
                cdr: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link(None),
                })))),
            })))),
        }),
        Token::Comma => Ok(Expression {
            content: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link(Some(Rc::new(ExpressionContent::Symbol("unquote")))),
                cdr: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link(None),
                })))),
            })))),
        }),
        Token::CommaAt => Ok(Expression {
            content: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                car: Link(Some(Rc::new(ExpressionContent::Symbol("unquote-splicing")))),
                cdr: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                    car: parse(buffer)?.content,
                    cdr: Link(None),
                })))),
            })))),
        }),
        Token::Dot => {
            unimplemented!()
        }
    }
}

fn parse_pair<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression<'a>> {
    if buffer.is_empty() {
        Err(ParseError::MissingCLoseParenthesis)
    } else {
        match *buffer.peek() {
            Token::CloseParenthesis => {
                buffer.pop();
                Ok(Expression {
                    content: Link(None),
                })
            }
            Token::Dot => {
                buffer.pop();
                let rest = parse(buffer)?;
                if buffer.is_empty() {
                    Err(ParseError::MissingCLoseParenthesis)
                } else {
                    match *buffer.pop() {
                        Token::CloseParenthesis => Ok(rest),
                        _ => Err(ParseError::TooMoreObjects),
                    }
                }
            }
            _ => {
                let first = parse(buffer)?;
                let rest = parse_pair(buffer)?;
                Ok(Expression {
                    content: Link(Some(Rc::new(ExpressionContent::PairLink(Pair {
                        car: first.content,
                        cdr: rest.content,
                    })))),
                })
            }
        }
    }
}

fn parse_vector<'a>(
    buffer: &mut TokenBuffer<'a>,
    mut vector: Vec<Link<'a>>,
) -> Result<Expression<'a>> {
    if buffer.is_empty() {
        Err(ParseError::MissingCLoseParenthesis)
    } else {
        match *buffer.peek() {
            Token::CloseParenthesis => {
                buffer.pop();
                Ok(Expression {
                    content: Link(Some(Rc::new(ExpressionContent::VectorLink(vector)))),
                })
            }
            _ => {
                let first = parse(buffer)?;
                vector.push(first.content.clone());
                parse_vector(buffer, vector)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize;

    use super::*;

    #[test]
    fn test_parse() {
        let mut buffer = tokenize("(#t #t #t)").unwrap();
        let expression = parse(&mut buffer).unwrap();
        // assert_eq!(expression.car().car().car().content, Some(1));
        println!("{:?}", expression);
        let mut buffer = tokenize("(1 2 3)").unwrap();
        println!("{:?}", buffer);
        println!("{:?}", parse(&mut buffer).unwrap());
        let mut buffer = tokenize("(2.3@4 2.3+5i +i)").unwrap();
        println!("{:?}", buffer);
        println!("{:?}", parse(&mut buffer).unwrap());
        println!("quote parse: {:?}", parse(&mut tokenize("'x").unwrap()));
        println!("parse empty: {:?}", parse(&mut tokenize("").unwrap()));
        println!(
            "string parse: {:?}",
            parse(&mut tokenize(r#""sdfsdf sdf\n sdfsd sdf \x03B1; \" asd""#).unwrap()).unwrap()
        );
        println!(
            "list parse: {}",
            parse(&mut tokenize("(1 2 3)").unwrap()).unwrap()
        );
        println!(
            "vector parse: {}",
            parse(&mut tokenize("#(1 2 3)").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_dotted_pair() {
        assert_eq!(
            parse(&mut tokenize("(1 . 2 3)").unwrap()),
            Err(ParseError::TooMoreObjects)
        );
        assert_eq!(
            parse(&mut tokenize("(1 2 3").unwrap()),
            Err(ParseError::MissingCLoseParenthesis)
        );
        // assert_eq!(
        //     parse(&mut tokenize("(1 2 3))").unwrap()),
        //     Err(ParseError::MissingOpenParenthesis)
        // );
        assert_eq!(
            parse(&mut tokenize("(1 2 3 4 . 5)").unwrap()),
            parse(&mut tokenize("(1 . (2 . (3 . (4 . 5))))").unwrap())
        );
        assert_eq!(
            parse(&mut tokenize("(a b c d e)").unwrap()),
            parse(&mut tokenize("(a . (b . (c . (d . (e . ())))))").unwrap())
        );
        println!(
            "dotted pair parse: {}",
            parse(&mut tokenize("(1 2 3 4 . 5)").unwrap()).unwrap()
        );
    }
}
