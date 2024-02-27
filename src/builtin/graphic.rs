use crate::{
    canvas::Canvas,
    data_model::{Expression, Link, Value},
    error::{invalid_number, ApplyError, InvalidArgument},
    number::Number,
};

// Turtle motion
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

pub fn right(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 1 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "right".to_string(),
            1,
            args.len(),
        ))?
    } else {
        let angle = args[0].as_number().ok_or(invalid_number(&args[0]))?;
        canvas.rotate(-angle.try_into()?);
        Ok(Value::Void)
    }
}

pub fn left(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 1 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "left".to_string(),
            1,
            args.len(),
        ))?
    } else {
        let angle = args[0].as_number().ok_or(invalid_number(&args[0]))?;
        canvas.rotate(angle.try_into()?);
        Ok(Value::Void)
    }
}

pub fn setposition(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 2 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "setposition".to_string(),
            2,
            args.len(),
        ))?
    } else {
        let x = args[0].as_number().ok_or(invalid_number(&args[0]))?;
        let y = args[1].as_number().ok_or(invalid_number(&args[1]))?;
        canvas.mov(x.try_into()?, -y.try_into()?);
        Ok(Value::Void)
    }
}

// tell turtle state

pub fn position(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "position".to_string(),
            0,
            args.len(),
        ))?
    } else {
        let x: Number = canvas.content.borrow().x.into();
        let y: Number = (-canvas.content.borrow().y).into();
        let pair_link = Link::new_pair(x.into(), Link::new_pair(y.into(), Link::Nil));
        Ok(pair_link.into())
    }
}

// Pen control

pub fn begin_fill(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "begin-fill".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.begin_fill();
        Ok(Value::Void)
    }
}

pub fn end_fill(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "end-fill".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.end_fill();
        Ok(Value::Void)
    }
}

pub fn penup(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "penup".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.penup();
        Ok(Value::Void)
    }
}

pub fn pendown(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    if args.len() != 0 {
        Err(InvalidArgument::InvalidNumberOfArguments(
            "pendown".to_string(),
            0,
            args.len(),
        ))?
    } else {
        canvas.pendown();
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
