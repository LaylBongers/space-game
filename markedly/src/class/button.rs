use nalgebra::{Point2, Vector2};

use class::{ComponentClass};
use template::{ComponentTemplate};
use render::{Renderer};
use {Color};

pub struct ButtonClass {
    background_color: Option<Color>,
    text_color: Color,
    text: Option<String>,
}

impl ButtonClass {
    pub fn new<R: Renderer>(
        template: &ComponentTemplate
    ) -> Result<Box<ComponentClass<R>>, String> {
        let background_color = template.attribute_optional("background-color", |v| v.as_color())?;
        let text_color = template.attribute(
            "text-color", |v| v.as_color(), Color::new_u8(0, 0, 0)
        )?;
        let text = template.attribute_optional("text", |v| v.as_string())?;

        Ok(Box::new(ButtonClass {
            background_color,
            text_color,
            text,
        }))
    }
}

impl<R: Renderer> ComponentClass<R> for ButtonClass {
    fn render(
        &self, renderer: &R, context: &mut R::Context, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), R::Error> {
        if let Some(background_color) = self.background_color {
            renderer.rectangle(context, position, size, background_color)?;
        }

        if let Some(ref text) = self.text {
            renderer.text(context, &text, position, size, self.text_color)?;
        }

        Ok(())
    }
}
