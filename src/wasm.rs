use wasm_bindgen::prelude::*;

use crate::{
    builtin::graphic::{
        backward, begin_fill, end_fill, forward, heading, hide_turtle, is_visible, left, pendown,
        penup, position, reset, right, setheading, setposition, show_turtle,
    },
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

#[wasm_bindgen]
pub struct Interpreter {
    frame: Frame,
    canvas: Canvas,
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let (frame, canvas) = create_wasm_global_env();
        Self { frame, canvas }
    }

    pub fn eval(&mut self, input: String) -> Result<Output, String> {
        match interpret(&input, &mut self.frame) {
            Ok(value) => Ok(Output {
                console: value.to_string(),
                canvas: self.canvas.export(),
            }),
            Err(err) => Err(err.to_string()),
        }
    }
}

fn create_wasm_global_env() -> (Frame, Canvas) {
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

    let right_procedure = GraphicProcedure {
        name: "right",
        function: right,
        canvas: canvas.clone(),
    };

    frame.add_graphic(right_procedure, &["rt"]);

    let left_procedure = GraphicProcedure {
        name: "left",
        function: left,
        canvas: canvas.clone(),
    };

    frame.add_graphic(left_procedure, &["lt"]);

    let setposition_procedure = GraphicProcedure {
        name: "setposition",
        function: setposition,
        canvas: canvas.clone(),
    };

    frame.add_graphic(setposition_procedure, &["setpos", "goto"]);

    let setheading_procedure = GraphicProcedure {
        name: "setheading",
        function: setheading,
        canvas: canvas.clone(),
    };

    frame.add_graphic(setheading_procedure, &["seth"]);

    let position_procedure = GraphicProcedure {
        name: "position",
        function: position,
        canvas: canvas.clone(),
    };

    frame.add_graphic(position_procedure, &["pos"]);

    let heading_procedure = GraphicProcedure {
        name: "heading",
        function: heading,
        canvas: canvas.clone(),
    };

    frame.add_graphic(heading_procedure, &[]);

    let begin_fill_procedure = GraphicProcedure {
        name: "begin-fill",
        function: begin_fill,
        canvas: canvas.clone(),
    };

    frame.add_graphic(begin_fill_procedure, &[]);

    let end_fill_procedure = GraphicProcedure {
        name: "end-fill",
        function: end_fill,
        canvas: canvas.clone(),
    };

    frame.add_graphic(end_fill_procedure, &[]);

    let penup_procedure = GraphicProcedure {
        name: "penup",
        function: penup,
        canvas: canvas.clone(),
    };

    frame.add_graphic(penup_procedure, &["pu", "up"]);

    let pendown_procedure = GraphicProcedure {
        name: "pendown",
        function: pendown,
        canvas: canvas.clone(),
    };

    frame.add_graphic(pendown_procedure, &["pd", "down"]);

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
