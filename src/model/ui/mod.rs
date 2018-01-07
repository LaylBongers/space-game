use nalgebra::{Point2, Vector2};

pub struct Button {
    pub position: Point2<i32>,
    pub size: Vector2<i32>,
    pub pressed: bool,
}

impl Button {
    pub fn new(position: Point2<i32>, size: Vector2<i32>) -> Self {
        Button {
            position,
            size,
            pressed: false,
        }
    }
}
