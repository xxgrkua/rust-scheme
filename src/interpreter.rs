use crate::{
    data_model::{Frame, Value},
    error::Error,
    eval, parse, tokenize,
};

pub fn interpret(input: &str, frame: &mut Frame) -> Result<Value, Error> {
    let mut tokens = tokenize(input)?;
    let expression = parse(&mut tokens)?;
    Ok(eval(expression, frame, false)?)
}
