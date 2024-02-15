use wasm_bindgen::prelude::*;

use crate::{create_global_frame, interpret};

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type getInterpreter = () => (code: string) => string;

"#;

#[wasm_bindgen(js_name = "getInterpreter")]
pub fn get_interpreter() -> JsValue {
    let mut frame = create_global_frame();

    let cb = Closure::<dyn FnMut(String) -> Result<String, String>>::new(move |input: String| {
        match interpret(&input, &mut frame) {
            Ok(value) => Ok(value.to_string()),
            Err(err) => Err(err.to_string()),
        }
    });

    let ret = cb.as_ref().clone();

    cb.forget();

    ret
}
