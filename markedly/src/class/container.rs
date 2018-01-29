use std::error::{Error};

use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory};
use template::{ComponentTemplate};
use render::{Renderer};
use {Color};

/// A container component class, functions as a generic container for other components.
pub struct ContainerClass {
    background_color: Option<Color>,
}

impl ComponentClassFactory for ContainerClass {
    fn new(
        template: &ComponentTemplate
    ) -> Result<Self, String> {
        Ok(ContainerClass {
            background_color: template.attribute_optional("background-color", |v| v.as_color())?,
        })
    }
}

impl ComponentClass for ContainerClass {
    fn render(
        &self, renderer: &mut Renderer, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), Box<Error>> {
        if let Some(background_color) = self.background_color {
            renderer.rectangle(position, size, background_color)?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        self.background_color.is_some()
    }
}
