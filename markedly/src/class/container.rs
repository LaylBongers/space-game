use nalgebra::{Point2};

use class::{ComponentClass, ComponentClassFactory};
use render::{Renderer};
use scripting::{ScriptRuntime};
use {Attributes, Error, Color, ComponentAttributes, ComponentId};

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
        &self, id: ComponentId, attributes: &ComponentAttributes, renderer: &mut Renderer,
    ) -> Result<(), Error> {
        if let Some(color) = self.color {
            renderer.rectangle(id, Point2::new(0.0, 0.0), attributes.size, color)?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        self.color.is_some()
    }
}
