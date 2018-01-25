use nalgebra::{Point2, Vector2};
use template::{Template, ComponentInstance};

/// A generic runtime component.
pub struct Component {
    pub children: Vec<Component>,
    pub position: Point2<f32>,
    pub size: Vector2<f32>,
}

impl Component {
    /// Create a new component from a template.
    pub fn new(template: &Template, screen_size: Vector2<f32>) -> Result<Self, String> {
        Self::from_template(&template.root, screen_size)
    }

    fn from_template(
        template: &ComponentInstance, parent_size: Vector2<f32>
    ) -> Result<Self, String> {
        let position = template.argument(
            "position", |v| v.as_point(parent_size), Point2::new(0.0, 0.0)
        )?;
        let size = template.argument(
            "size", |v| v.as_vector(parent_size), parent_size
        )?;

        let mut children = Vec::new();
        for child_template in &template.children {
            children.push(Self::from_template(&child_template, size)?);
        }

        Ok(Component {
            children,
            position,
            size,
        })
    }
}
