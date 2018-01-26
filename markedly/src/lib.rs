//! A markup based UI engine.

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate nalgebra;

pub mod class;
pub mod template;
mod component;
mod renderer;

pub use component::{Component};
pub use renderer::{Renderer, render};
