use crate::{
    canvas::Canvas,
    data_model::Value,
    error::{ApplyError, InvalidArgument},
};

pub fn reset(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "#[clear]".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.reset();
        Ok(Value::Unspecified)
    }
}
