//! Component classes that define functionality and appearance

mod container;
mod classes;
mod button;

pub use self::container::{ContainerClass};
pub use self::classes::{ComponentClasses};
pub use self::button::{ButtonClass};

use nalgebra::{Point2, Vector2};

use render::{Renderer};

pub trait ComponentClass<R: Renderer> {
    fn render(
        &self, renderer: &R, context: &mut R::Context, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), R::Error>;
}
