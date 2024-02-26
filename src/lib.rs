mod builtin;
#[cfg(target_arch = "wasm32")]
mod canvas;
mod data_model;
mod error;
mod evaluator;
mod frame;
mod interpreter;
mod lexer;
mod number;
mod parser;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use evaluator::eval;
pub use frame::create_global_frame;
pub use interpreter::interpret;
pub use lexer::tokenize;
pub use parser::parse;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
