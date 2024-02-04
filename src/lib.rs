mod builtin;
mod data_model;
mod error;
mod evaluator;
mod frame;
mod lexer;
mod number;
mod parser;

use data_model::{Frame, Value};
use error::Error;
pub use evaluator::eval;
pub use frame::create_global_frame;
pub use lexer::tokenize;
pub use parser::parse;

pub fn scheme_eval(input: &str, frame: &mut Frame) -> Result<Value, Error> {
    let mut tokens = tokenize(input)?;
    let expression = parse(&mut tokens)?;
    Ok(eval(expression, frame, false)?)
}
