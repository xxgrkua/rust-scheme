use std::cmp::Ordering;

use crate::{
    data_model::{BuiltinProcedure, Value},
    error::{invalid_number, validate_number_of_arguments, ApplyError, InvalidArgument},
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
    validate_number_of_arguments("#[-]", 1, usize::MAX, args.len())?;
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
    validate_number_of_arguments("#[/]", 1, usize::MAX, args.len())?;
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

fn equal(args: Vec<Value>) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[=]", 2, usize::MAX, args.len())?;
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

pub const MATH_EQUAL: BuiltinProcedure = BuiltinProcedure {
    name: "=",
    function: equal,
};

fn less_than(args: Vec<Value>) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[<]", 2, usize::MAX, args.len())?;
    let (first, rest) = split_value(&args);
    let mut first = first.as_number().ok_or(invalid_number(&first))?;
    for arg in rest {
        let number = arg.as_number().ok_or(invalid_number(&arg))?;
        if let Some(Ordering::Less) = first.partial_cmp(number) {
            first = number;
        } else {
            return Ok(false.into());
        }
    }
    Ok(true.into())
}

pub const LESS_THAN: BuiltinProcedure = BuiltinProcedure {
    name: "<",
    function: less_than,
};

fn less_than_or_equal(args: Vec<Value>) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[<=]", 2, usize::MAX, args.len())?;
    let (first, rest) = split_value(&args);
    let mut first = first.as_number().ok_or(invalid_number(&first))?;
    for arg in rest {
        let number = arg.as_number().ok_or(invalid_number(&arg))?;
        if let Some(Ordering::Less | Ordering::Equal) = first.partial_cmp(number) {
            first = number;
        } else {
            return Ok(false.into());
        }
    }
    Ok(true.into())
}

pub const LESS_THAN_OR_EQUAL: BuiltinProcedure = BuiltinProcedure {
    name: "<=",
    function: less_than_or_equal,
};

fn greater_than(args: Vec<Value>) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[>]", 2, usize::MAX, args.len())?;
    let (first, rest) = split_value(&args);
    let mut first = first.as_number().ok_or(invalid_number(&first))?;
    for arg in rest {
        let number = arg.as_number().ok_or(invalid_number(&arg))?;
        if let Some(Ordering::Greater) = first.partial_cmp(number) {
            first = number;
        } else {
            return Ok(false.into());
        }
    }
    Ok(true.into())
}

pub const GREATER_THAN: BuiltinProcedure = BuiltinProcedure {
    name: ">",
    function: greater_than,
};

fn greater_than_or_equal(args: Vec<Value>) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[>=]", 2, usize::MAX, args.len())?;
    let (first, rest) = split_value(&args);
    let mut first = first.as_number().ok_or(invalid_number(&first))?;
    for arg in rest {
        let number = arg.as_number().ok_or(invalid_number(&arg))?;
        if let Some(Ordering::Greater | Ordering::Equal) = first.partial_cmp(number) {
            first = number;
        } else {
            return Ok(false.into());
        }
    }
    Ok(true.into())
}

pub const GREATER_THAN_OR_EQUAL: BuiltinProcedure = BuiltinProcedure {
    name: ">=",
    function: greater_than_or_equal,
};
