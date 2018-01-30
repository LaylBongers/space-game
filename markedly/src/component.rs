use nalgebra::{Point2, Vector2};

use class::{ComponentClasses, ComponentClass};
use template::{ComponentTemplate, Style};
use {ComponentId, ComponentEvents, ComponentEventsSender, Attributes};

/// A runtime component.
pub struct Component {
    pub(crate) class: Box<ComponentClass>,
    pub(crate) style_class: Option<String>,
    pub(crate) events_sender: ComponentEventsSender,

    pub(crate) children: Vec<ComponentId>,
    pub(crate) position: Point2<f32>,
    pub(crate) size: Vector2<f32>,
}

impl Component {
    pub(crate) fn from_template(
        template: &ComponentTemplate, style: &Style,
        parent_size: Vector2<f32>, classes: &ComponentClasses,
        events: &ComponentEvents,
    ) -> Result<Self, String> {
        let attributes = Attributes::resolve(template, style);

        let class = classes.create(template, &attributes)?;

        let position = attributes.attribute(
            "position", |v| v.as_point(parent_size), Point2::new(0.0, 0.0)
        )?;
        let size = attributes.attribute(
            "size", |v| v.as_vector(parent_size), parent_size
        )?;

        Ok(Component {
            class,
            style_class: template.style_class.clone(),
            events_sender: events.create_sender(),

            children: Vec::new(),
            position,
            size,
        })
    }
}
