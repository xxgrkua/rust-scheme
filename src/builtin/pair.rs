use crate::{
    data_model::{BuiltinProcedure, Value},
    error::{ApplyError, InvalidArgument},
};

pub(crate) const IS_PAIR: BuiltinProcedure = BuiltinProcedure {
    name: "pair?",
    function: is_pair,
};

fn is_pair(args: Vec<Value>) -> Result<Value, ApplyError> {
    if args.len() != 1 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "pair?".to_string(),
            1,
            args.len(),
        ))?
    } else {
        Ok(args[0].as_pair().is_some().into())
    }
}

pub(crate) const CAR: BuiltinProcedure = BuiltinProcedure {
    name: "car",
    function: car,
};

fn car(args: Vec<Value>) -> Result<Value, ApplyError> {
    if args.len() != 1 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "car".to_string(),
            1,
            args.len(),
        ))?
    } else {
        args[0]
            .as_pair()
            .map(|pair| pair.car().into())
            .ok_or_else(|| {
                InvalidArgument::InvalidType(args[0].to_string(), "pair".to_string()).into()
            })
    }
}
