use crate::{
    builtin::math::ADD,
    data_model::{Frame, Procedure, Value},
};

pub fn create_global_frame() -> Frame {
    let mut frame = Frame::new();
    frame.define("+", Value::Procedure(Procedure::Builtin(ADD)));
    frame
}
