use nalgebra::{Point2, Vector2};

use class::{ComponentClass};
use template::{ComponentTemplate};
use {Renderer, Color};

pub struct ContainerClass {
    background_color: Option<Color>,
}

impl ContainerClass {
    pub fn new<R: Renderer>(
        template: &ComponentTemplate
    ) -> Result<Box<ComponentClass<R>>, String> {
        let background_color = template.attribute_optional("background-color", |v| v.as_color())?;

        Ok(Box::new(ContainerClass {
            background_color,
        }))
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
