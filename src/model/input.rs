use nalgebra::{Point2};

pub struct InputState {
    pub mouse_down: bool,
    pub hovered_tile: Point2<i32>,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            mouse_down: false,
            hovered_tile: Point2::new(0, 0),
        }
    }
}
