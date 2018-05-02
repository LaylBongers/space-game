extern crate alga;
extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate pathfinding as pathfindingc;
extern crate metrohash;
extern crate mtk_tilegame;

pub mod object_class;
pub mod state;
pub mod pathfinding;

use {
    object_class::{ObjectClassId},
};

#[derive(Debug, PartialEq)]
pub enum Error {
    ObjectClassNotFound(ObjectClassId),
    Tiles(mtk_tilegame::tiles::Error),
}

impl From<mtk_tilegame::tiles::Error> for Error {
    fn from(error: mtk_tilegame::tiles::Error) -> Self {
        Error::Tiles(error)
    }
}
