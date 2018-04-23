use nalgebra::{Point2};
use {ObjectClassId};

#[derive(Deserialize, Serialize)]
pub struct BuildState {
    pub drag: BuildDrag,
    pub choice: BuildChoice,
}

#[derive(Deserialize, Serialize)]
pub enum BuildDrag {
    Hovering { position: Option<Point2<i32>> },
    Dragging { start: Point2<i32>, end: Point2<i32> },
}

#[derive(Copy, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum BuildChoice {
    None,
    Floor,
    Object(ObjectClassId),
    Destroy,
    DestroyAll,
}

pub fn normalize_area(start: Point2<i32>, end: Point2<i32>) -> (Point2<i32>, Point2<i32>) {
    let min_x = start.x.min(end.x);
    let min_y = start.y.min(end.y);
    let max_x = start.x.max(end.x);
    let max_y = start.y.max(end.y);
    (Point2::new(min_x, min_y), Point2::new(max_x + 1, max_y + 1))
}
