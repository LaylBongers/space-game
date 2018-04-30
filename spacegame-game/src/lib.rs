extern crate alga;
extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate pathfinding as pathfindingc;
extern crate metrohash;

pub mod object_class;
pub mod state;
pub mod pathfinding;

use {
    nalgebra::{Point2},
    object_class::{ObjectClassId},
};

#[derive(Debug, PartialEq)]
pub enum Error {
    ObjectClassNotFound(ObjectClassId),
    OutOfBounds { position: Point2<i32> },
}
