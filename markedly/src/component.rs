use nalgebra::{Point2, Vector2};

use class::{ComponentClasses, ComponentClass};
use template::{Template, ComponentInstance};
use {Renderer};

/// A generic runtime component.
pub struct Component<R: Renderer> {
    pub class: Box<ComponentClass<R>>,
    pub children: Vec<Component<R>>,
    pub position: Point2<f32>,
    pub size: Vector2<f32>,
}

impl<R: Renderer> Component<R> {
    /// Create a new component from a template.
    pub fn new(
        template: &Template, screen_size: Vector2<f32>, classes: &ComponentClasses<R>
    ) -> Result<Self, String> {
        Self::from_template(&template.root, screen_size, classes)
    }

    fn from_template(
        template: &ComponentInstance, parent_size: Vector2<f32>, classes: &ComponentClasses<R>,
    ) -> Result<Self, String> {
        let class = classes.create(&template.class)?;

        let position = template.argument(
            "position", |v| v.as_point(parent_size), Point2::new(0.0, 0.0)
        )?;
        let size = template.argument(
            "size", |v| v.as_vector(parent_size), parent_size
        )?;

        let mut children = Vec::new();
        for child_template in &template.children {
            children.push(Self::from_template(&child_template, size, classes)?);
        }

        Ok(Component {
            class,
            children,
            position,
            size,
        })
    }
}
