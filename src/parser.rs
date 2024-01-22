use std::{cell::RefCell, rc::Rc};

use crate::{lexer::Token, number::Number};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Number(Number),
    Symbol(&'a str),
    Pair(Box<Expression<'a>>, RefCell<Rc<Expression<'a>>>),
    Nil,
}

pub fn parse(expr: Vec<Token>) -> Expression {
    let expr = Expression::Number(Number::Integer(0));

    expr
}
