use nalgebra::{Point2, Vector2};

use class::{ComponentClass, ComponentClassFactory};
use render::{Renderer};
use scripting::{ScriptRuntime};
use {ComponentEvents, Attributes, Error, Color};

pub struct ButtonAttributes {
    color: Option<Color>,
    color_hovering: Option<Color>,
    text_color: Color,
    text: Option<String>,
    on_pressed: Option<String>,
}

impl ButtonAttributes {
    pub fn load(attributes: &Attributes, runtime: &ScriptRuntime) -> Result<Self, Error> {
        Ok(ButtonAttributes {
            color: attributes.attribute_optional(
                "color", |v| v.as_color(runtime)
            )?,
            color_hovering: attributes.attribute_optional(
                "color-hovering", |v| v.as_color(runtime)
            )?,
            text_color: attributes.attribute(
                "text-color", |v| v.as_color(runtime), Color::new_u8(0, 0, 0, 255)
            )?,
            text: attributes.attribute_optional("text", |v| v.as_string(runtime))?,
            on_pressed: attributes.attribute_optional("on-pressed", |v| v.as_string(runtime))?,
        })
    }
}

/// A button component class, raises events on click.
pub struct ButtonClass {
    attributes: ButtonAttributes,
    hovering: bool,
}

impl ComponentClassFactory for ButtonClass {
    fn new(attributes: &Attributes, runtime: &ScriptRuntime) -> Result<Self, Error> {
        Ok(ButtonClass {
            attributes: ButtonAttributes::load(attributes, runtime)?,
            hovering: false,
        })
    }
}

impl ComponentClass for ButtonClass {
    fn update_attributes(
        &mut self, attributes: &Attributes, runtime: &ScriptRuntime,
    ) -> Result<(), Error> {
        self.attributes = ButtonAttributes::load(attributes, runtime)?;
        Ok(())
    }

    fn render(
        &self, renderer: &mut Renderer, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), Error> {
        let attribs = &self.attributes;

        let current_color = if self.hovering && attribs.color_hovering.is_some() {
            attribs.color_hovering
        } else {
            attribs.color
        };

        if let Some(current_color) = current_color {
            renderer.rectangle(position, size, current_color)?;
        }

        if let Some(ref text) = attribs.text {
            renderer.text(&text, position, size, attribs.text_color)?;
        }

        Ok(())
    }

    fn is_capturing_cursor(&self) -> bool {
        true
    }

    fn hover_start_event(&mut self, _sender: &ComponentEvents) {
        self.hovering = true;
    }

    fn hover_end_event(&mut self, _sender: &ComponentEvents) {
        self.hovering = false;
    }

    fn pressed_event(&mut self, sender: &ComponentEvents) {
        if let Some(event) = self.attributes.on_pressed.clone() {
            sender.raise(event);
        }
    }
}
