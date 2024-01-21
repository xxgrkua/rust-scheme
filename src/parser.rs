use crate::lexer::Token;

pub enum Expression<'a> {
    Integer(i32),
    Float(f32),
    Symbol(&'a str),
}

pub fn parse(expr: Vec<Token>) -> Expression {
    let expr = Expression::Integer(0);

    expr
}
