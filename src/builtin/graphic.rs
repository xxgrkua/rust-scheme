use crate::{
    canvas::Canvas,
    data_model::{Link, Value},
    error::{invalid_number, validate_number_of_arguments, ApplyError},
    number::Number,
};

// Turtle motion
// move and draw

pub fn forward(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[forward]", 1, 1, args.len())?;
    let distance = args[0].as_number().ok_or(invalid_number(&args[0]))?;
    canvas.forward(distance.try_into()?);
    Ok(Value::Void)
}

pub fn backward(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[backward]", 1, 1, args.len())?;
    let distance = args[0].as_number().ok_or(invalid_number(&args[0]))?;
    canvas.forward(-distance.try_into()?);
    Ok(Value::Void)
}

pub fn right(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[right]", 1, 1, args.len())?;
    let angle = args[0].as_number().ok_or(invalid_number(&args[0]))?;
    canvas.rotate(-angle.try_into()?);
    Ok(Value::Void)
}

pub fn left(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[left]", 1, 1, args.len())?;
    let angle = args[0].as_number().ok_or(invalid_number(&args[0]))?;
    canvas.rotate(angle.try_into()?);
    Ok(Value::Void)
}

pub fn setposition(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[setposition]", 2, 2, args.len())?;
    let x = args[0].as_number().ok_or(invalid_number(&args[0]))?;
    let y = args[1].as_number().ok_or(invalid_number(&args[1]))?;
    canvas.mov(x.try_into()?, -y.try_into()?);
    Ok(Value::Void)
}

pub fn setheading(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[setheading]", 1, 1, args.len())?;
    let angle = args[0].as_number().ok_or(invalid_number(&args[0]))?;
    let angle: f64 = angle.try_into()?;
    canvas.abs_rotate(90.0 - angle);
    Ok(Value::Void)
}

// tell turtle state

pub fn position(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[position]", 0, 0, args.len())?;
    let x: Number = canvas.content.borrow().x.into();
    let y: Number = (-canvas.content.borrow().y).into();
    let pair_link = Link::new_pair(x.into(), Link::new_pair(y.into(), Link::Nil));
    Ok(pair_link.into())
}

pub fn heading(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[heading]", 0, 0, args.len())?;
    let angle: Number = (90.0 + canvas.content.borrow().angle).into();
    Ok(angle.into())
}

// Pen control

pub fn begin_fill(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[begin-fill]", 0, 0, args.len())?;
    canvas.begin_fill();
    Ok(Value::Void)
}

pub fn end_fill(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[end-fill]", 0, 0, args.len())?;
    canvas.end_fill();
    Ok(Value::Void)
}

pub fn penup(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[penup]", 0, 0, args.len())?;
    canvas.penup();
    Ok(Value::Void)
}

pub fn pendown(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[pendown]", 0, 0, args.len())?;
    canvas.pendown();
    Ok(Value::Void)
}

pub fn reset(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[reset]", 0, 0, args.len())?;
    canvas.reset();
    Ok(Value::Void)
}

// turtle state

pub fn show_turtle(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[showturtle]", 0, 0, args.len())?;
    canvas.show_turtle();
    Ok(Value::Void)
}

pub fn hide_turtle(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[hideturtle]", 0, 0, args.len())?;
    canvas.hide_turtle();
    Ok(Value::Void)
}

pub fn is_visible(args: Vec<Value>, canvas: &mut Canvas) -> Result<Value, ApplyError> {
    validate_number_of_arguments("#[visible?]", 0, 0, args.len())?;
    Ok(canvas.is_visible().into())
}
