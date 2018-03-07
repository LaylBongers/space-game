//! A markup based UI engine.

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate nalgebra;
extern crate palette;
extern crate metrohash;
extern crate rlua;

pub mod class;
pub mod input;
pub mod render;
pub mod scripting;
pub mod template;

mod attributes;
mod component;
mod error;
mod events;
mod ui;

use component::{Component};

pub use component::{ComponentAttributes};
pub use attributes::{Attributes};
pub use error::{Error};
pub use events::{ComponentEvents};
pub use ui::{Ui, UiContext, ComponentId};

/// Re-export of palette's color for convenience so you don't have to add palette to your own
/// crate unless you need more complex color functionality.
pub type Color = ::palette::Srgba;
