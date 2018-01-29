use std::error::{Error};

use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory};
use template::{ComponentTemplate};
use render::{Renderer};
use {Color, ComponentEventsSender};

/// A button component class, raises events on click.
pub struct ButtonClass {
    background_color: Option<Color>,
    text_color: Color,
    text: Option<String>,
    on_pressed: Option<String>,
}

impl ComponentClassFactory for ButtonClass {
    fn new(
        template: &ComponentTemplate
    ) -> Result<Self, String> {
        Ok(ButtonClass {
            background_color: template.attribute_optional("background-color", |v| v.as_color())?,
            text_color: template.attribute(
                "text-color", |v| v.as_color(), Color::new_u8(0, 0, 0)
            )?,
            text: template.attribute_optional("text", |v| v.as_string())?,
            on_pressed: template.attribute_optional("on-pressed", |v| v.as_string())?,
        })
    }
}

impl ComponentClass for ButtonClass {
    fn render(
        &self, renderer: &mut Renderer, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), Box<Error>> {
        if let Some(background_color) = self.background_color {
            renderer.rectangle(position, size, background_color)?;
        }

        if let Some(ref text) = self.text {
            renderer.text(&text, position, size, self.text_color)?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        true
    }

    fn pressed_event(&self, sender: &ComponentEventsSender) {
        if let Some(event) = self.on_pressed.clone() {
            sender.raise(event);
        }
    }
}
