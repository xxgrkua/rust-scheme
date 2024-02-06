use crate::{
    data_model::{
        BuiltinProcedure, Expression, ExpressionContent, Frame, LambdaProcedure, Link, Procedure,
        SpecialForm, Thunk, Value,
    },
    error::{invalid_symbol, ApplyError, EvalError, InvalidArgument},
};
use phf::phf_map;

const SPECIAL_FORMS: phf::Map<&str, SpecialForm> = phf_map! {
    "and"    => SpecialForm::And,
    "begin"  => SpecialForm::Begin,
    "case"   => SpecialForm::Case,
    "cond"   => SpecialForm::Cond,
    "define" => SpecialForm::Define,
    "delay"  => SpecialForm::Delay,
    "do"     => SpecialForm::Do,
    "if"     => SpecialForm::If,
    "lambda" => SpecialForm::Lambda,
    "let-syntax" => SpecialForm::LetSyntax,
    "let"    => SpecialForm::Let,
    "let*"   => SpecialForm::LetStar,
    "letrec-syntax" => SpecialForm::LetRecSyntax,
    "letrec" => SpecialForm::LetRec,
    "or"     => SpecialForm::Or,
    "quasiquote" => SpecialForm::QuasiQuote,
    "quote"  => SpecialForm::Quote,
    "set!"   => SpecialForm::Set,
    "syntax-rules" => SpecialForm::SyntaxRules,
    "unquote-splicing" => SpecialForm::UnquoteSplicing,
    "unquote" => SpecialForm::Unquote,
};

pub fn eval(
    expression: Expression,
    frame: &mut Frame,
    tail_content: bool,
) -> Result<Value, EvalError> {
    match expression.content.as_expression_content() {
        Some(ExpressionContent::PairLink(pair)) => {
            if tail_content {
                return Ok(Value::Thunk(Thunk {
                    content: expression.content.clone(),
                    frame: frame.content,
                }));
            }
            if let Some(symbol) = pair.car.as_symbol() {
                if SPECIAL_FORMS.contains_key(symbol) {
                    let special_form = SPECIAL_FORMS[symbol];
                    return Ok(special_form.apply(pair.cdr(), frame)?);
                }
            }
            let operator = eval(pair.car().into(), frame, false)?;
            if let Value::Procedure(procedure) = operator {
                let mut operands = vec![];
                for expression_content in pair.cdr.iter() {
                    operands.push(eval(expression_content.clone().into(), frame, false)?);
                }
                Ok(procedure.apply(operands, frame)?)
            } else {
                Err(ApplyError::InvalidProcedure(operator.to_string()))?
            }
        }
        Some(ExpressionContent::Promise(promise)) => {
            unimplemented!()
        }
        Some(ExpressionContent::Symbol(symbol)) => match frame.lookup(symbol) {
            Some(value) => Ok(value.clone()),
            None => Err(EvalError::UnknownIdentifier(symbol.to_string())),
        },
        _ => Ok(expression.into()),
    }
}

impl SpecialForm {
    pub fn apply(&self, args: Link, frame: &mut Frame) -> Result<Value, EvalError> {
        match self {
            Self::Define => do_define_form(args, frame),
            Self::Quote => do_quote_form(args, frame),
            Self::Lambda => do_lambda_form(args, frame),
            _ => {
                unimplemented!()
            }
        }
    }
}

fn validate_number_of_arguments(
    name: &str,
    least_expected: usize,
    most_expected: usize,
    actual: usize,
) -> Result<(), InvalidArgument> {
    if (least_expected == most_expected) && (actual != least_expected) {
        Err(InvalidArgument::InvalidNumberOfArguments(
            name.to_string(),
            least_expected,
            actual,
        ))
    } else if (most_expected == usize::MAX) && (actual < least_expected) {
        Err(InvalidArgument::TooFewArguments(
            name.to_string(),
            least_expected,
            actual,
        ))
    } else {
        Ok(())
    }
}

fn do_define_form(args: Link, frame: &mut Frame) -> Result<Value, EvalError> {
    validate_number_of_arguments("define", 2, usize::MAX, args.len())?;
    let pair = args.as_pair().unwrap();
    match pair.car.as_expression_content() {
        Some(ExpressionContent::Symbol(name)) => {
            validate_number_of_arguments("define", 2, 2, args.len())?;
            let value = eval(pair.cdr.as_pair().unwrap().car().into(), frame, false)?;
            frame.define(name, value);
            Ok(pair.car().into())
        }
        Some(ExpressionContent::PairLink(params)) => {
            if let Some(name) = params.car.as_symbol() {
                let value = do_lambda_form(Link::new_pair(params.cdr(), pair.cdr()), frame)?;
                frame.define(name, value);
                Ok(params.car().into())
            } else {
                Err(invalid_symbol(&params.car))?
            }
        }
        _ => Err(InvalidArgument::InvalidType(
            pair.car.to_string(),
            "symbol or pair".to_string(),
        ))?,
    }
}

fn do_lambda_form(args: Link, frame: &mut Frame) -> Result<Value, EvalError> {
    validate_number_of_arguments("lambda", 2, usize::MAX, args.len())?;
    unimplemented!()
}

fn do_quote_form(args: Link, _: &mut Frame) -> Result<Value, EvalError> {
    validate_number_of_arguments("quote", 1, 1, args.len())?;
    if args.len() != 1 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "quote".to_string(),
            1,
            args.len(),
        ))?
    } else {
        Ok(args.as_pair().unwrap().car().into())
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
