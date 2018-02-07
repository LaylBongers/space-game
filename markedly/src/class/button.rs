use std::error::{Error};

use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory};
use render::{Renderer};
use {Color, ComponentEventsClient, Attributes};

/// A button component class, raises events on click.
pub struct ButtonClass {
    color: Option<Color>,
    color_hovering: Option<Color>,
    text_color: Color,
    text: Option<String>,
    on_pressed: Option<String>,

    hovering: bool,
}

impl ComponentClassFactory for ButtonClass {
    fn new(attributes: &Attributes) -> Result<Self, String> {
        Ok(ButtonClass {
            color: attributes.attribute_optional(
                "color", |v| v.as_color()
            )?,
            color_hovering: attributes.attribute_optional(
                "color-hovering", |v| v.as_color()
            )?,
            text_color: attributes.attribute(
                "text-color", |v| v.as_color(), Color::new_u8(0, 0, 0)
            )?,
            text: attributes.attribute_optional("text", |v| v.as_string())?,
            on_pressed: attributes.attribute_optional("on-pressed", |v| v.as_string())?,

            hovering: false,
        })
    }
}

impl ComponentClass for ButtonClass {
    fn render(
        &self, renderer: &mut Renderer, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), Box<Error>> {
        let current_color = if self.hovering && self.color_hovering.is_some() {
            self.color_hovering
        } else {
            self.color
        };

        if let Some(current_color) = current_color {
            renderer.rectangle(position, size, current_color)?;
        }

        if let Some(ref text) = self.text {
            renderer.text(&text, position, size, self.text_color)?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        true
    }

    fn hover_start_event(&mut self, _sender: &ComponentEventsClient) {
        self.hovering = true;
    }

    fn hover_end_event(&mut self, _sender: &ComponentEventsClient) {
        self.hovering = false;
    }

    fn pressed_event(&mut self, sender: &ComponentEventsClient) {
        if let Some(event) = self.on_pressed.clone() {
            sender.raise(event);
        }
    }
}
