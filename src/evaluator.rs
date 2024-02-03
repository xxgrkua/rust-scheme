use crate::{
    data_model::{
        BuiltinProcedure, Expression, ExpressionContent, Frame, LambdaProcedure, Link, Procedure,
        Thunk, Value,
    },
    error::{ApplyError, EvalError},
};

// const SPECIAL_FORMS:

pub fn eval(
    expression: Expression,
    frame: &mut Frame,
    tail_content: bool,
) -> Result<Value, EvalError> {
    match &expression.content {
        Link::More(content) => match content.as_ref() {
            ExpressionContent::PairLink(pair) => {
                if tail_content {
                    return Ok(Value::Thunk(Thunk {
                        content: expression.content.clone(),
                        frame: frame.content,
                    }));
                }
                let operator = eval(
                    Expression {
                        content: pair.car.clone(),
                    },
                    frame,
                    false,
                )?;
                if let Value::Procedure(procedure) = operator {
                    let mut operands = vec![];
                    for expression_content in pair.cdr.outer_iter() {
                        operands.push(eval(
                            Expression {
                                content: Link::More(expression_content.clone()),
                            },
                            frame,
                            false,
                        )?);
                    }
                    Ok(procedure.apply(operands, frame)?)
                } else {
                    Err(ApplyError::InvalidProcedure(operator.to_string()))?
                }
            }
            ExpressionContent::Promise(promise) => {
                unimplemented!()
            }
            ExpressionContent::Symbol(symbol) => match frame.lookup(symbol) {
                Some(value) => Ok(value.clone()),
                None => Err(EvalError::UnknownIdentifier(symbol.to_string())),
            },
            _ => Ok(Value::Expression(expression)),
        },
        _ => Ok(Value::Expression(expression)),
    }
}

impl BuiltinProcedure {
    pub fn apply(&self, args: Vec<Value>, _: &mut Frame) -> Result<Value, ApplyError> {
        (self.function)(args)
    }
}

impl LambdaProcedure {
    pub fn apply(&self, args: Vec<Value>, frame: &mut Frame) -> Result<Value, ApplyError> {
        unimplemented!()
    }
}

impl Procedure {
    pub fn apply(&self, args: Vec<Value>, frame: &mut Frame) -> Result<Value, ApplyError> {
        match self {
            Procedure::Builtin(builtin) => builtin.apply(args, frame),
            Procedure::Lambda(lambda) => lambda.apply(args, frame),
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;
    use crate::data_model::{ExpressionContent, Link};
    use crate::frame::create_global_frame;
    use crate::lexer::tokenize;
    use crate::number::Number;
    use crate::parser::parse;

    #[test]
    fn test_eval() {
        let mut frame = create_global_frame();
        let expression = parse(&mut tokenize("(+ 1 2)").unwrap()).unwrap();
        let result = eval(expression, &mut frame, false).unwrap();
        assert_eq!(
            result,
            Value::Expression(Expression {
                content: Link::More(Rc::new(ExpressionContent::Number(Number::Integer(3))))
            })
        );
    }
}
