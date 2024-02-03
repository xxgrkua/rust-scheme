use crate::{
    builtin::math::{ADD, DIV, MUL, SUB},
    data_model::{Frame, Procedure, Value},
};

pub fn create_global_frame() -> Frame {
    let mut frame = Frame::new();
    frame.define("+", Value::Procedure(Procedure::Builtin(ADD)));
    frame.define("-", Value::Procedure(Procedure::Builtin(SUB)));
    frame.define("*", Value::Procedure(Procedure::Builtin(MUL)));
    frame.define("/", Value::Procedure(Procedure::Builtin(DIV)));
    frame
}
