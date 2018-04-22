extern crate metrohash;
extern crate nalgebra;
extern crate palette;

pub mod panels;
pub mod rendering;
mod error;
mod ui;

pub use {
    error::{Error},
    ui::{Ui, PanelId},
};

pub enum Size {
    Max,
    Absolute(f32),
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

// Convenience re-exports so for basic usage you don't need the dependencies
pub use {
    nalgebra::{Point2, Vector2},
    palette::{Srgba},
};
