//! A markup based UI engine.

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate nalgebra;

pub mod template;
mod component;

pub use component::{Component};
