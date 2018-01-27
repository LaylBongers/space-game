//! A markup based UI engine.

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate nalgebra;
extern crate palette;

pub mod class;
pub mod render;
pub mod template;
mod component;
mod value;

pub use component::{Component};
pub use value::{Value, Color};
