use wasm_bindgen::prelude::*;

use crate::{
    builtin::graphic::{backward, forward, hide_turtle, is_visible, reset, show_turtle},
    canvas::{Canvas, Path},
    create_global_frame,
    data_model::{Frame, GraphicProcedure},
    interpret,
};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct SVGPath {
    d: String,
    stroke: String,
    fill: String,
}

#[wasm_bindgen]
impl SVGPath {
    #[wasm_bindgen(getter)]
    pub fn d(&self) -> String {
        self.d.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn stroke(&self) -> String {
        self.stroke.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn fill(&self) -> String {
        self.fill.clone()
    }
}

impl Path {
    fn export(&self) -> SVGPath {
        SVGPath {
            d: self
                .moves
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            stroke: self.stroke.clone(),
            fill: self.fill.clone(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CanvasState {
    #[wasm_bindgen(readonly)]
    pub x: f64,

    #[wasm_bindgen(readonly)]
    pub y: f64,

    paths: Vec<SVGPath>,

    #[wasm_bindgen(readonly)]
    pub rotation: f64,

    bg_color: String,

    #[wasm_bindgen(readonly)]
    pub visible: bool,
}

#[wasm_bindgen]
impl CanvasState {
    #[wasm_bindgen(getter)]
    pub fn paths(&self) -> Vec<SVGPath> {
        self.paths.clone()
    }

    #[wasm_bindgen(getter, js_name = "bgColor")]
    pub fn bg_color(&self) -> String {
        self.bg_color.clone()
    }
}

impl Canvas {
    fn export(&self) -> CanvasState {
        CanvasState {
            x: self.content.borrow().x,
            y: self.content.borrow().y,
            paths: self
                .content
                .borrow()
                .paths
                .iter()
                .map(|p| p.export())
                .collect(),
            rotation: self.content.borrow().angle,
            bg_color: self.content.borrow().bg_color.clone(),
            visible: self.content.borrow().turtle_visible,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Output {
    console: String,
    canvas: CanvasState,
}

#[wasm_bindgen]
impl Output {
    #[wasm_bindgen(getter)]
    pub fn console(&self) -> String {
        self.console.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn canvas(&self) -> CanvasState {
        self.canvas.clone()
    }
}

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

export declare function getInterpreter(): (code: string) => Output;

"#;

#[wasm_bindgen(js_name = "getInterpreter", skip_typescript)]
pub fn get_interpreter() -> JsValue {
    let (mut frame, canvas) = create_wasm_global_frame();

    let cb = Closure::<dyn FnMut(String) -> Result<Output, String>>::new(move |input: String| {
        match interpret(&input, &mut frame) {
            Ok(value) => Ok(Output {
                console: value.to_string(),
                canvas: canvas.export(),
            }),
            Err(err) => Err(err.to_string()),
        }
    });

    let ret = cb.as_ref().clone();

    cb.forget();

    ret
}
