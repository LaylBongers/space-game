extern crate cgmath;
extern crate ggez;
#[macro_use] extern crate slog;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate pathfinding as pathfindingc;
extern crate metrohash;
extern crate lagato;

pub mod object_class;
pub mod state;
pub mod pathfinding;

use {
    object_class::{ObjectClassId},
    state::ship::{TaskId},
};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidClassId(ObjectClassId),
    InvalidTaskId(TaskId),
    Tiles(lagato::grid::Error),
}

impl From<lagato::grid::Error> for Error {
    fn from(error: lagato::grid::Error) -> Self {
        Error::Tiles(error)
    }
}
