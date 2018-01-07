use nalgebra::{Point2};

pub struct InputState {
    pub build_down: bool,
    pub move_down: bool,
    pub hovered_tile: Option<Point2<i32>>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            build_down: false,
            move_down: false,
            hovered_tile: None,
        }
    }
}
