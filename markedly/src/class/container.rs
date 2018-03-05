use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory};
use render::{Renderer};
use scripting::{ScriptRuntime};
use {Color, Attributes, Error};

/// A container component class, functions as a generic container for other components.
pub struct ContainerClass {
    color: Option<Color>,
}

impl ComponentClassFactory for ContainerClass {
    fn new(attributes: &Attributes, runtime: &ScriptRuntime) -> Result<Self, Error> {
        Ok(ContainerClass {
            color: attributes.attribute_optional("color", |v| v.as_color(runtime))?,
        })
    }
}

impl ComponentClass for ContainerClass {
    fn update_attributes(
        &mut self, attributes: &Attributes, runtime: &ScriptRuntime,
    ) -> Result<(), Error> {
        self.color = attributes.attribute_optional("color", |v| v.as_color(runtime))?;
        Ok(())
    }

    fn render(
        &self, renderer: &mut Renderer, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), Error> {
        if let Some(color) = self.color {
            renderer.rectangle(position, size, color)?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        self.color.is_some()
    }
}
