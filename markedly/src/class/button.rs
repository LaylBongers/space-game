use nalgebra::{Point2, Vector2};

use class::{ComponentClass};
use {Renderer};

pub struct ButtonClass {
}

impl ButtonClass {
    pub fn new<R: Renderer>() -> Box<ComponentClass<R>> {
        Box::new(ButtonClass {
        })
    }
}

impl<R: Renderer> ComponentClass<R> for ButtonClass {
    fn render(
        &self, renderer: &R, context: &mut R::Context, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), R::Error> {
        renderer.rectangle(context, position, size)?;

        Ok(())
    }
}
