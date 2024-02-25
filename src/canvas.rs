use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    AbsoluteMove(f64, f64),
    RelativeMove(f64, f64),
    AbsoluteLine(f64, f64),
    RelativeLine(f64, f64),
    CompletePath,
    AbsoluteArc(f64, f64, i32, bool, bool, f64, f64),
    RelativeArc(f64, f64, i32, bool, bool, f64, f64),
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::AbsoluteMove(x, y) => write!(f, "M {} {}", x, y),
            Move::RelativeMove(dx, dy) => write!(f, "m {} {}", dx, dy),
            Move::AbsoluteLine(x, y) => write!(f, "L {} {}", x, y),
            Move::RelativeLine(dx, dy) => write!(f, "l {} {}", dx, dy),
            Move::CompletePath => write!(f, "Z"),
            Move::AbsoluteArc(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y) => {
                write!(
                    f,
                    "A {} {} {} {} {} {} {}",
                    rx,
                    ry,
                    x_axis_rotation,
                    (*large_arc_flag) as i32,
                    (*sweep_flag) as i32,
                    x,
                    y
                )
            }
            Move::RelativeArc(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy) => {
                write!(
                    f,
                    "a {} {} {} {} {} {} {}",
                    rx,
                    ry,
                    x_axis_rotation,
                    (*large_arc_flag) as i32,
                    (*sweep_flag) as i32,
                    dx,
                    dy
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Path {
    stroke: String,
    fill: String,
    moves: Vec<Move>,
}

impl Default for Path {
    fn default() -> Self {
        Self {
            stroke: "black".to_string(),
            fill: "transparent".to_string(),
            moves: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    content: Rc<RefCell<CanvasContent>>,
}

#[derive(Debug, Clone, PartialEq)]
struct CanvasContent {
    x: f64,
    y: f64,
    angle: f64,
    bg_color: String,
    paths: Vec<Path>,
    fill_path: Option<Path>,
    pen_down: bool,
    turtle_visible: bool,
    size: f64,
}

impl Default for Canvas {
    fn default() -> Self {
        let mut canvas = Self {
            content: Rc::new(RefCell::new(CanvasContent {
                x: 0.0,
                y: 0.0,
                angle: -90.0,
                bg_color: "white".to_string(),
                paths: vec![],
                fill_path: None,
                pen_down: true,
                turtle_visible: true,
                size: 1.0,
            })),
        };
        canvas.new_path();
        canvas
    }
}

impl Canvas {
    fn new_path(&mut self) {
        let mut path = Path::default();
        path.moves.push(Move::AbsoluteMove(
            self.content.borrow().x,
            self.content.borrow().y,
        ));
        self.content.borrow_mut().paths.push(path);
    }

    fn reset(&mut self) {
        self.content.borrow_mut().x = 0.0;
        self.content.borrow_mut().y = 0.0;
        self.content.borrow_mut().angle = -90.0;
        self.content.borrow_mut().bg_color = "white".to_string();
        self.content.borrow_mut().paths = vec![];
        self.content.borrow_mut().fill_path = None;
        self.content.borrow_mut().pen_down = true;
        self.content.borrow_mut().turtle_visible = true;
        self.content.borrow_mut().size = 1.0;
        self.new_path();
    }

    fn set_color(&mut self, color: &str) {
        self.new_path();
        self.content.borrow_mut().paths.last_mut().unwrap().stroke = color.to_string();
    }

    fn mov(&mut self, x: f64, y: f64) {
        if self.content.borrow().pen_down {
            self.content
                .borrow_mut()
                .paths
                .last_mut()
                .unwrap()
                .moves
                .push(Move::AbsoluteLine(x, y));
        } else {
            self.content
                .borrow_mut()
                .paths
                .last_mut()
                .unwrap()
                .moves
                .push(Move::AbsoluteMove(x, y));
        }
        if let Some(fill_move) = &mut self.content.borrow_mut().fill_path {
            fill_move.moves.push(Move::AbsoluteLine(x, y));
        }
        self.content.borrow_mut().x = x;
        self.content.borrow_mut().y = y;
    }

    fn begin_fill(&mut self) {
        let mut path = Path::default();
        path.fill = self.content.borrow().paths.last().unwrap().stroke.clone();
        self.content.borrow_mut().fill_path = Some(path);
    }

    fn end_fill(&mut self) {
        if let Some(fill_path) = self.content.borrow_mut().fill_path.take() {
            self.content
                .borrow_mut()
                .paths
                .insert(self.content.borrow().paths.len() - 1, fill_path);
        }
    }

    fn set_bg(&mut self, color: &str) {
        self.content.borrow_mut().bg_color = color.to_string();
    }

    fn rotate(&mut self, theta: f64) {
        self.content.borrow_mut().angle -= theta;
        self.content.borrow_mut().angle %= 360.0;
    }

    fn abs_rotate(&mut self, theta: f64) {
        self.content.borrow_mut().angle = -theta % 360.0;
    }

    fn pendown(&mut self) {
        self.content.borrow_mut().pen_down = true;
    }

    fn penup(&mut self) {
        self.content.borrow_mut().pen_down = false;
    }

    fn show_turtle(&mut self) {
        self.content.borrow_mut().turtle_visible = true;
    }

    fn hide_turtle(&mut self) {
        self.content.borrow_mut().turtle_visible = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_size() {
        println!("Size of Move: {}", std::mem::size_of::<Move>());
    }
}
