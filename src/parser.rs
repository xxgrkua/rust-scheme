use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    error::ParseError,
    lexer::{Token, TokenBuffer},
    number::Number,
};

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct Expression<'a> {
    content: Link<'a>,
}

impl<'a> Expression<'a> {
    pub fn car(&self) -> Self {
        if let Some(expression) = &self.content {
            if let ExpressionContent::PairLink(pair) = &*expression.borrow() {
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

type Link<'a> = Option<Rc<RefCell<ExpressionContent<'a>>>>;

#[derive(Debug, Clone)]
enum ExpressionContent<'a> {
    Number(Number),
    String(String),
    Boolean(bool),
    Symbol(&'a str),
    PairLink(Pair<'a>),
}

#[derive(Debug, Clone)]
struct Pair<'a> {
    car: Link<'a>,
    cdr: Link<'a>,
}

pub fn parse<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression<'a>> {
    match *buffer.pop() {
        Token::Identifier(identifier) => {
            unimplemented!()
        }
        Token::Boolean(value) => match value {
            "#t" | "#true" => Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::Boolean(true)))),
            }),
            "#f" | "#false" => Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::Boolean(false)))),
            }),
            _ => unreachable!(),
        },
        Token::Number(number) => {
            unimplemented!()
        }
        Token::Comment(_) => unimplemented!(),
        Token::OpenParenthesis => parse_tail(buffer),
        _ => {
            unimplemented!()
        }
    }
}

fn parse_tail<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression<'a>> {
    if buffer.is_empty() {
        Err(ParseError::UnexpectedEOF)
    } else {
        match *buffer.peek() {
            Token::CloseParenthesis => Ok(Expression { content: None }),
            _ => {
                let first = parse(buffer)?;
                let rest = parse_tail(buffer)?;
                Ok(Expression {
                    content: Some(Rc::new(RefCell::new(ExpressionContent::PairLink(Pair {
                        car: first.content.clone(),
                        cdr: rest.content.clone(),
                    })))),
                })
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
        println!("{:?}", expression)
    }
}
