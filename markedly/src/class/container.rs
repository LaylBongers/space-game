use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory};
use template::{ComponentTemplate};
use render::{Renderer};
use {Color};

/// A container component class, functions as a generic container for other components.
pub struct ContainerClass {
    background_color: Option<Color>,
}

impl<R: Renderer> ComponentClassFactory<R> for ContainerClass {
    fn new(
        template: &ComponentTemplate
    ) -> Result<Self, String> {
        let background_color = template.attribute_optional("background-color", |v| v.as_color())?;

        Ok(ContainerClass {
            background_color,
        })
    }
}

impl<R: Renderer> ComponentClass<R> for ContainerClass {
    fn render(
        &self, renderer: &R, context: &mut R::Context, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), R::Error> {
        if let Some(background_color) = self.background_color {
            renderer.rectangle(context, position, size, background_color)?;
        }

        Ok(())
    }
}
