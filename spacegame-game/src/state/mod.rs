pub mod ship;
mod camera;

pub use self::camera::{Camera};

use nalgebra::{Point2};
use {ObjectClassId};

pub struct BuildInputState {
    pub state: BuildState,
    pub choice: BuildChoice,
}

impl BuildInputState {
    pub fn new() -> Self {
        BuildInputState {
            state: BuildState::Hovering { position: None, },
            choice: BuildChoice::None,
        }
    }
}

pub enum BuildState {
    Hovering { position: Option<Point2<i32>> },
    Dragging { start: Point2<i32>, end: Point2<i32> },
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BuildChoice {
    None,
    Floor,
    Object(ObjectClassId),
    Destroy,
    DestroyAll,
}
