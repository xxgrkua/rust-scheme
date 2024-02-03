use std::rc::Rc;

use crate::{
    data_model::{BuiltinProcedure, Expression, ExpressionContent, Link, Value},
    error::{ApplyError, InvalidArgument},
    number::Number,
};

fn split_value(args: &[Value]) -> (&Value, &[Value]) {
    (args.first().unwrap(), &args[1..])
}

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

pub(crate) const SUB: BuiltinProcedure = BuiltinProcedure {
    name: "-",
    function: sub,
};

fn sub(args: Vec<Value>) -> Result<Value, ApplyError> {
    if args.is_empty() {
        Err(InvalidArgument::TooFewArguments("#[-]".to_string()))?
    } else {
        let (first, rest) = split_value(&args);
        let mut difference = if let Value::Expression(Expression {
            content: Link::More(expression_content),
        }) = first
        {
            if let ExpressionContent::Number(number) = expression_content.as_ref() {
                *number
            } else {
                return Err(InvalidArgument::InvalidNumber(first.to_string()))?;
            }
        } else {
            return Err(InvalidArgument::InvalidNumber(first.to_string()))?;
        };
        if rest.len() > 0 {
            for arg in rest {
                if let Value::Expression(Expression {
                    content: Link::More(expression_content),
                }) = arg
                {
                    if let ExpressionContent::Number(number) = expression_content.as_ref() {
                        difference = difference - number;
                    } else {
                        return Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
                    }
                } else {
                    return Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
                }
            }
        } else {
            difference = -difference;
        }
        Ok(Value::Expression(Expression {
            content: Link::More(Rc::new(ExpressionContent::Number(difference))),
        }))
    }
}

pub(crate) const MUL: BuiltinProcedure = BuiltinProcedure {
    name: "*",
    function: mul,
};

fn mul(args: Vec<Value>) -> Result<Value, ApplyError> {
    let mut product = Number::Integer(1);
    for arg in &args {
        if let Value::Expression(Expression {
            content: Link::More(expression_content),
        }) = arg
        {
            if let ExpressionContent::Number(number) = expression_content.as_ref() {
                product = product * number;
            } else {
                Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
            }
        } else {
            Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
        }
    }
    Ok(Value::Expression(Expression {
        content: Link::More(Rc::new(ExpressionContent::Number(product))),
    }))
}

pub(crate) const DIV: BuiltinProcedure = BuiltinProcedure {
    name: "/",
    function: div,
};

fn div(args: Vec<Value>) -> Result<Value, ApplyError> {
    if args.is_empty() {
        Err(InvalidArgument::TooFewArguments("#[/]".to_string()))?
    } else {
        let (first, rest) = split_value(&args);
        let mut quotient = if let Value::Expression(Expression {
            content: Link::More(expression_content),
        }) = first
        {
            if let ExpressionContent::Number(number) = expression_content.as_ref() {
                *number
            } else {
                return Err(InvalidArgument::InvalidNumber(first.to_string()))?;
            }
        } else {
            return Err(InvalidArgument::InvalidNumber(first.to_string()))?;
        };
        if rest.len() > 0 {
            for arg in rest {
                if let Value::Expression(Expression {
                    content: Link::More(expression_content),
                }) = arg
                {
                    if let ExpressionContent::Number(number) = expression_content.as_ref() {
                        if *number == Number::Integer(0) {
                            return Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
                        }
                        quotient = quotient / number;
                    } else {
                        return Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
                    }
                } else {
                    return Err(InvalidArgument::InvalidNumber(arg.to_string()))?;
                }
            }
        } else {
            if quotient == Number::Integer(0) {
                return Err(InvalidArgument::ZeroDivisor)?;
            }
            quotient = Number::Integer(1) / quotient;
        }
        Ok(Value::Expression(Expression {
            content: Link::More(Rc::new(ExpressionContent::Number(quotient))),
        }))
    }
}
