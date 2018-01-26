use nalgebra::{Point2, Vector2};

use class::{ComponentClass};
use {Renderer};

pub struct ContainerClass {
}

impl ContainerClass {
    pub fn new<R: Renderer>() -> Box<ComponentClass<R>> {
        Box::new(ContainerClass {
        })
    }
}

impl<R: Renderer> ComponentClass<R> for ContainerClass {
    fn render(
        &self, _renderer: &R, _context: &mut R::Context, _position: Point2<f32>, _size: Vector2<f32>
    ) -> Result<(), R::Error> {
        Ok(())
    }
}
