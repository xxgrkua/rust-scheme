use std::rc::Rc;

use crate::{
    data_model::{BuiltinProcedure, Expression, ExpressionContent, Link, Value},
    error::{ApplyError, InvalidArgument},
    number::Number,
};

pub(crate) const ADD: BuiltinProcedure = BuiltinProcedure {
    name: "+",
    function: add,
};

fn add(args: Vec<Value>) -> Result<Value, ApplyError> {
    let mut sum = Number::Integer(0);
    for arg in &args {
        if let Value::Expression(Expression {
            content: Link::More(expression_content),
        }) = arg
        {
            if let ExpressionContent::Number(number) = expression_content.as_ref() {
                sum = sum + number;
            } else {
                Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
            }
        } else {
            Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
        }
    }
    Ok(Value::Expression(Expression {
        content: Link::More(Rc::new(ExpressionContent::Number(sum))),
    }))
}
