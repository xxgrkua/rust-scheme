use crate::{
    data_model::{BuiltinProcedure, Value},
    error::{validate_number_of_arguments, ApplyError, InvalidArgument},
};

pub(crate) const IS_PAIR: BuiltinProcedure = BuiltinProcedure {
    name: "pair?",
    function: is_pair,
};

fn is_pair(args: Vec<Value>) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[pair?]", 1, 1, args.len())?;
    Ok(args[0].as_pair().is_some().into())
}

pub(crate) const CAR: BuiltinProcedure = BuiltinProcedure {
    name: "car",
    function: car,
};

fn car(args: Vec<Value>) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[car]", 1, 1, args.len())?;
    args[0]
        .as_pair()
        .map(|pair| pair.car().into())
        .ok_or_else(|| InvalidArgument::InvalidType(args[0].to_string(), "pair".to_string()).into())
}
