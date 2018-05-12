extern crate alga;
extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate pathfinding as pathfindingc;
extern crate metrohash;
extern crate mtk;

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
    Tiles(mtk::grid::Error),
}

impl From<mtk::grid::Error> for Error {
    fn from(error: mtk::grid::Error) -> Self {
        Error::Tiles(error)
    }
}
