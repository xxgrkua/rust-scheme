use crate::{
    builtin::{
        math::{ADD, DIV, MUL, SUB},
        pair::{CAR, IS_PAIR},
    },
    data_model::Frame,
};

pub fn create_global_frame() -> Frame {
    let mut frame = Frame::new();

    // numbers builtins
    frame.add_builtin(ADD);
    frame.add_builtin(SUB);
    frame.add_builtin(MUL);
    frame.add_builtin(DIV);

    // pair builtins
    frame.add_builtin(IS_PAIR);
    frame.add_builtin(CAR);

    frame
}
