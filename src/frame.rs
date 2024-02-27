use crate::{
    builtin::{
        math::{
            ADD, DIV, GREATER_THAN, GREATER_THAN_OR_EQUAL, LESS_THAN, LESS_THAN_OR_EQUAL,
            MATH_EQUAL, MUL, SUB,
        },
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
    frame.add_builtin(MATH_EQUAL);
    frame.add_builtin(LESS_THAN);
    frame.add_builtin(LESS_THAN_OR_EQUAL);
    frame.add_builtin(GREATER_THAN);
    frame.add_builtin(GREATER_THAN_OR_EQUAL);

    // pair builtins
    frame.add_builtin(IS_PAIR);
    frame.add_builtin(CAR);

    frame
}
