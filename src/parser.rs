use std::{cell::RefCell, rc::Rc};

use crate::{lexer::Token, number::Number};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Number(Number),
    Symbol(&'a str),
    Pair(Link<'a>),
    Nil,
}

#[derive(Debug, Clone)]
enum Link<'a> {
    End(Rc<Expression<'a>>),
    More(Rc<Node<'a>>),
}

#[derive(Debug, Clone)]
struct Node<'a> {
    car: Expression<'a>,
    cdr: Link<'a>,
}

pub fn parse(expr: Vec<Token>) -> Expression {
    let expr = Expression::Number(Number::Integer(0));

    expr
}
