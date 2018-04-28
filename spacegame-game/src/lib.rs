extern crate alga;
extern crate ggez;
extern crate nalgebra;
#[macro_use] extern crate slog;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate pathfinding as pathfindingc;
extern crate metrohash;

pub mod state;
pub mod pathfinding;
mod object_classes;

pub use self::object_classes::{ObjectClass, ObjectClasses, ObjectClassId, ObjectBehavior, DoorObjectBehavior};
