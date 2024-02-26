use wasm_bindgen::prelude::*;

use crate::{
    builtin::graphic::{backward, forward, hide_turtle, is_visible, reset, show_turtle},
    canvas::Canvas,
    create_global_frame,
    data_model::{Frame, GraphicProcedure},
    interpret,
};

fn create_wasm_global_frame() -> (Frame, Canvas) {
    let mut frame = create_global_frame();
    let canvas = Canvas::default();

    let forward_procedure = GraphicProcedure {
        name: "forward",
        function: forward,
        canvas: canvas.clone(),
    };

    frame.add_graphic(forward_procedure, &["fd"]);

    let backward_procedure = GraphicProcedure {
        name: "backward",
        function: backward,
        canvas: canvas.clone(),
    };

    frame.add_graphic(backward_procedure, &["bk", "back"]);

    let reset_procedure = GraphicProcedure {
        name: "reset",
        function: reset,
        canvas: canvas.clone(),
    };

    frame.add_graphic(reset_procedure, &[]);

    let show_turtle_procedure = GraphicProcedure {
        name: "showturtle",
        function: show_turtle,
        canvas: canvas.clone(),
    };

    frame.add_graphic(show_turtle_procedure, &["st"]);

    let hide_turtle_procedure = GraphicProcedure {
        name: "hideturtle",
        function: hide_turtle,
        canvas: canvas.clone(),
    };

    frame.add_graphic(hide_turtle_procedure, &["ht"]);

    let is_visible_procedure = GraphicProcedure {
        name: "visible?",
        function: is_visible,
        canvas: canvas.clone(),
    };

    frame.add_graphic(is_visible_procedure, &[]);

    (frame, canvas)
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export declare function getInterpreter(): (code: string) => string;

"#;

#[wasm_bindgen(js_name = "getInterpreter", skip_typescript)]
pub fn get_interpreter() -> JsValue {
    let (mut frame, canvas) = create_wasm_global_frame();

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
