use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{lexer::Token, number::Number};

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
    Symbol(&'a str),
    PairLink(Pair<'a>),
}

#[derive(Debug, Clone)]
struct Pair<'a> {
    car: Link<'a>,
    cdr: Link<'a>,
}

pub fn parse(expr: Vec<Token>) -> Expression {
    unimplemented!();
}
