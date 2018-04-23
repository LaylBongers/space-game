mod panelbox;
mod size;

// Convenience re-exports so for basic usage you don't need the dependencies
pub use {
    nalgebra::{Point2, Vector2},
    palette::{Srgba},
};

pub use self::{
    size::{PanelSize, AxisSize},
    panelbox::{PanelBox},
};

pub struct PanelText {
    pub text: String,
    pub size: u32,
}

impl PanelText {
    pub fn new<S: Into<String>>(text: S, size: u32) -> PanelText {
        PanelText {
            text: text.into(),
            size,
        }
    }
}

pub enum Orientation {
    Horizontal,
    Vertical,
}
