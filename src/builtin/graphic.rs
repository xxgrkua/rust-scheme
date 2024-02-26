use crate::{
    canvas::Canvas,
    data_model::Value,
    error::{invalid_number, ApplyError, InvalidArgument},
};

// move and draw

pub fn forward(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 1 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "forward".to_string(),
            1,
            args.len(),
        ))?
    } else {
        let distance = args[0].as_number().ok_or(invalid_number(&args[0]))?;
        canvas.forward(distance.try_into()?);
        Ok(Value::Void)
    }
}

pub fn backward(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 1 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "backward".to_string(),
            1,
            args.len(),
        ))?
    } else {
        let distance = args[0].as_number().ok_or(invalid_number(&args[0]))?;
        canvas.forward(-distance.try_into()?);
        Ok(Value::Void)
    }
}

pub fn reset(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "reset".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.reset();
        Ok(Value::Void)
    }
}

// turtle state

pub fn show_turtle(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "showturtle".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.show_turtle();
        Ok(Value::Void)
    }
}

pub fn hide_turtle(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "hideturtle".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.hide_turtle();
        Ok(Value::Void)
    }
}

pub fn is_visible(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "visible?".to_string(),
            0,
            args.len(),
        ))?
    } else {
        Ok(canvas.is_visible().into())
    }
}
