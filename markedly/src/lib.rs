//! A markup based UI engine.

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate nalgebra;
extern crate palette;
extern crate metrohash;

pub mod class;
pub mod input;
pub mod render;
pub mod template;

mod component;
mod events;
mod ui;
mod value;

pub use component::{Component};
pub use events::{ComponentEvents, ComponentEventsSender};
pub use ui::{Ui, ComponentId};
pub use value::{Value, Color};
