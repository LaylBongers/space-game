use nalgebra::{Point2, Vector2};

use class::{ComponentClasses, ComponentClass};
use template::{ComponentTemplate};
use {ComponentId};

/// A runtime component.
pub struct Component {
    pub(crate) class: Box<ComponentClass>,
    pub(crate) style_class: Option<String>,
    pub(crate) children: Vec<ComponentId>,
    pub(crate) position: Point2<f32>,
    pub(crate) size: Vector2<f32>,
}

impl Component {
    pub(crate) fn from_template(
        template: &ComponentTemplate, parent_size: Vector2<f32>, classes: &ComponentClasses,
    ) -> Result<Self, String> {
        let class = classes.create(&template)?;

        let position = template.attribute(
            "position", |v| v.as_point(parent_size), Point2::new(0.0, 0.0)
        )?;
        let size = template.attribute(
            "size", |v| v.as_vector(parent_size), parent_size
        )?;

        Ok(Component {
            class,
            style_class: template.style_class.clone(),
            children: Vec::new(),
            position,
            size,
        })
    }
}
