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
    object_class::{ObjectClassId},
};

#[derive(Debug)]
pub enum Error {
    ObjectClassNotFound(ObjectClassId)
}
