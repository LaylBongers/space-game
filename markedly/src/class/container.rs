use std::error::{Error};

use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory};
use render::{Renderer};
use {Color, Attributes};

/// A container component class, functions as a generic container for other components.
pub struct ContainerClass {
    color: Option<Color>,
}

impl ComponentClassFactory for ContainerClass {
    fn new(attributes: &Attributes) -> Result<Self, String> {
        Ok(ContainerClass {
            color: attributes.attribute_optional("color", |v| v.as_color())?,
        })
    }
}

impl ComponentClass for ContainerClass {
    fn render(
        &self, renderer: &mut Renderer, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), Box<Error>> {
        if let Some(color) = self.color {
            renderer.rectangle(position, size, color)?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        self.color.is_some()
    }
}
