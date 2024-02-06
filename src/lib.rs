mod builtin;
mod data_model;
mod error;
mod evaluator;
mod frame;
mod lexer;
mod number;
mod parser;

// #[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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

// pub fn test_evel() -> Box<dyn FnMut(String) -> Result<String, String>> {
//     let mut frame = create_global_frame();
//     return Box::new(move |input| match scheme_eval(&input, &mut frame) {
//         Ok(value) => Ok(value.to_string()),
//         Err(err) => Err(err.to_string()),
//     });
// }
// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn scheme_eval_wasm() -> JsValue {
    let mut frame = create_global_frame();

    let cb = Closure::<dyn FnMut(String) -> Result<String, String>>::new(move |input: String| {
        match scheme_eval(&input, &mut frame) {
            Ok(value) => Ok(value.to_string()),
            Err(err) => Err(err.to_string()),
        }
    });

    let ret = cb.as_ref().clone();

    cb.forget();

    ret
}
