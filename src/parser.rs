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
    VectorLink(Vec<Link<'a>>),
}

#[derive(Debug, Clone)]
struct Pair<'a> {
    car: Link<'a>,
    cdr: Link<'a>,
}

pub fn parse<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression<'a>> {
    match *buffer.pop() {
        Token::Identifier(identifier) => Ok(Expression {
            content: Some(Rc::new(RefCell::new(ExpressionContent::Symbol(identifier)))),
        }),
        Token::Boolean(value) => match value {
            "#t" | "#true" => Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::Boolean(true)))),
            }),
            "#f" | "#false" => Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::Boolean(false)))),
            }),
            _ => unreachable!(),
        },
        Token::Number(number) => Ok(Expression {
            content: Some(Rc::new(RefCell::new(ExpressionContent::Number(
                Number::try_from(number)?,
            )))),
        }),
        Token::String(string) => {
            unimplemented!()
        }
        Token::Comment(_) => parse(buffer),
        Token::OpenParenthesis => parse_pair(buffer),
        Token::CloseParenthesis => {
            unimplemented!()
        }
        Token::VectorOpen => {
            unimplemented!();
            // TODO: this is wrong. To evaluate to a vector, it must be quoted.
            let vector = vec![];
            parse_vector(buffer, vector)
        }
        Token::ByteVectorOpen => unimplemented!(),
        Token::Quote => {
            let rest = parse(buffer)?;
            Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::PairLink(Pair {
                    car: Some(Rc::new(RefCell::new(ExpressionContent::Symbol("quote")))),
                    cdr: rest.content.clone(),
                })))),
            })
        }
        Token::BackQuote => {
            let rest = parse(buffer)?;
            Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::PairLink(Pair {
                    car: Some(Rc::new(RefCell::new(ExpressionContent::Symbol(
                        "quasiquote",
                    )))),
                    cdr: rest.content.clone(),
                })))),
            })
        }
        Token::Comma => {
            let rest = parse(buffer)?;
            Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::PairLink(Pair {
                    car: Some(Rc::new(RefCell::new(ExpressionContent::Symbol("unquote")))),
                    cdr: rest.content.clone(),
                })))),
            })
        }
        Token::CommaAt => {
            let rest = parse(buffer)?;
            Ok(Expression {
                content: Some(Rc::new(RefCell::new(ExpressionContent::PairLink(Pair {
                    car: Some(Rc::new(RefCell::new(ExpressionContent::Symbol(
                        "unquote-splicing",
                    )))),
                    cdr: rest.content.clone(),
                })))),
            })
        }
        Token::Dot => {
            unimplemented!()
        }
    }
}

fn parse_pair<'a>(buffer: &mut TokenBuffer<'a>) -> Result<Expression<'a>> {
    if buffer.is_empty() {
        Err(ParseError::UnexpectedEOF)
    } else {
        match *buffer.peek() {
            Token::CloseParenthesis => {
                buffer.pop();
                Ok(Expression { content: None })
            }
            _ => {
                let first = parse(buffer)?;
                let rest = parse_pair(buffer)?;
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

fn parse_vector<'a>(
    buffer: &mut TokenBuffer<'a>,
    mut vector: Vec<Link<'a>>,
) -> Result<Expression<'a>> {
    if buffer.is_empty() {
        Err(ParseError::UnexpectedEOF)
    } else {
        match *buffer.peek() {
            Token::CloseParenthesis => {
                buffer.pop();
                Ok(Expression {
                    content: Some(Rc::new(RefCell::new(ExpressionContent::VectorLink(vector)))),
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
    }
}
