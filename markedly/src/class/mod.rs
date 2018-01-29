//! Component classes that define functionality and appearance

mod container;
mod classes;
mod button;

pub use self::container::{ContainerClass};
pub use self::classes::{ComponentClasses, ComponentClassFactory};
pub use self::button::{ButtonClass};

use nalgebra::{Point2, Vector2};

use render::{Renderer};

/// The class of a component, defines specific appearance and functionality in response to user
/// input.
pub trait ComponentClass<R: Renderer> {
    /// Renders the component.
    fn render(
        &self, renderer: &R, context: &mut R::Context, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), R::Error>;

    /// Called when the component is clicked or tapped.
    fn pressed_event(&mut self) {}
}
