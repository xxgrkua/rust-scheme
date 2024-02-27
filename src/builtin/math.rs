use crate::{
    data_model::{BuiltinProcedure, Value},
    error::{invalid_number, ApplyError, InvalidArgument},
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
        if let Some(number) = arg.as_number() {
            sum = sum + number;
        } else {
            Err(invalid_number(&arg))?;
        }
    }
    Ok(sum.into())
}

pub(crate) const SUB: BuiltinProcedure = BuiltinProcedure {
    name: "-",
    function: sub,
};

fn sub(args: Vec<Value>) -> Result<Value, ApplyError> {
    if args.is_empty() {
        Err(InvalidArgument::TooFewArguments("-".to_string(), 1, 0))?
    } else {
        let (first, rest) = split_value(&args);
        let mut difference = *first.as_number().ok_or(invalid_number(&first))?;
        if rest.len() > 0 {
            for arg in rest {
                if let Some(number) = arg.as_number() {
                    difference = difference - number;
                } else {
                    return Err(invalid_number(&arg))?;
                }
            }
        } else {
            difference = -difference;
        }
        Ok(difference.into())
    }
}

pub(crate) const MUL: BuiltinProcedure = BuiltinProcedure {
    name: "*",
    function: mul,
};

fn mul(args: Vec<Value>) -> Result<Value, ApplyError> {
    let mut product = Number::Integer(1);
    for arg in &args {
        if let Some(number) = arg.as_number() {
            product = product * number;
        } else {
            Err(invalid_number(&arg))?;
        }
    }
    Ok(product.into())
}

pub(crate) const DIV: BuiltinProcedure = BuiltinProcedure {
    name: "/",
    function: div,
};

fn div(args: Vec<Value>) -> Result<Value, ApplyError> {
    if args.is_empty() {
        Err(InvalidArgument::TooFewArguments("/".to_string(), 1, 0))?
    } else {
        let (first, rest) = split_value(&args);
        let mut quotient = *first.as_number().ok_or(invalid_number(&first))?;
        if rest.len() > 0 {
            for arg in rest {
                if let Some(number) = arg.as_number() {
                    if *number == Number::Integer(0)
                        || *number == Number::Real(0.0)
                        || *number == Number::Complex(0.0, 0.0)
                    {
                        return Err(InvalidArgument::ZeroDivisor)?;
                    }
                    quotient = quotient / number;
                } else {
                    return Err(invalid_number(&arg))?;
                }
            }
        } else {
            if quotient == Number::Integer(0)
                || quotient == Number::Real(0.0)
                || quotient == Number::Complex(0.0, 0.0)
            {
                return Err(InvalidArgument::ZeroDivisor)?;
            }
            quotient = Number::Integer(1) / quotient;
        }
        Ok(quotient.into())
    }
}

fn equal(args: Vec<Value>) -> Result<Value, ApplyError> {
    if args.len() < 2 {
        Err(InvalidArgument::TooFewArguments(
            "=".to_string(),
            2,
            args.len(),
        ))?
    } else {
        let (first, rest) = split_value(&args);
        let first = first.as_number().ok_or(invalid_number(&first))?;
        for arg in rest {
            let number = arg.as_number().ok_or(invalid_number(&arg))?;
            if first != number {
                return Ok(false.into());
            }
        }
        Ok(true.into())
    }
}

pub const MATH_EQUAL: BuiltinProcedure = BuiltinProcedure {
    name: "=",
    function: equal,
};
