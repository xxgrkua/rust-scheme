use crate::{
    builtin::{
        math::{ADD, DIV, MUL, SUB},
        pair::{CAR, IS_PAIR},
    },
    data_model::Frame,
};

pub fn create_global_frame() -> Frame {
    let mut frame = Frame::new();
    println!("frame");
    frame.define("+", ADD.into());
    frame.define("-", SUB.into());
    frame.define("*", MUL.into());
    frame.define("/", DIV.into());

    frame.define("pair?", IS_PAIR.into());
    frame.define("car", CAR.into());
    frame
}
