//! Component classes that define functionality and appearance

mod container;
mod classes;
mod button;

pub use self::container::{ContainerClass};
pub use self::classes::{ComponentClasses, ComponentClassFactory};
pub use self::button::{ButtonClass};

use std::error::{Error};
use nalgebra::{Point2, Vector2};

use render::{Renderer};

/// The class of a component, defines specific appearance and functionality in response to user
/// input.
pub trait ComponentClass {
    /// Renders the component.
    fn render(
        &self, backend: &mut Renderer, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), Box<Error>>;

    /// Returns if this component class captures cursor events or not. Does not affect children.
    fn is_capturing_cursor(&self) -> bool { false }

    /// Called when the component is clicked or tapped.
    fn pressed_event(&self) {}
}
