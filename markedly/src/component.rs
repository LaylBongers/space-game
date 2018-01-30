use nalgebra::{Point2, Vector2};

use class::{ComponentClasses, ComponentClass};
use template::{ComponentTemplate, Style};
use {ComponentId, ComponentEvents, ComponentEventsSender, Attributes, Value};

/// A runtime component.
pub struct Component {
    pub(crate) class: Box<ComponentClass>,
    pub(crate) style_class: Option<String>,
    pub(crate) events_sender: ComponentEventsSender,

    pub(crate) children: Vec<ComponentId>,
    pub(crate) position: Point2<f32>,
    pub(crate) size: Vector2<f32>,
    docking: (Docking, Docking),
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

        let docking = attributes.attribute(
            "docking", |v| Docking::from_value(v), (Docking::Start, Docking::Start)
        )?;

        Ok(Component {
            class,
            style_class: template.style_class.clone(),
            events_sender: events.create_sender(),

            children: Vec::new(),
            position,
            size,
            docking,
        })
    }

    pub(crate) fn compute_position(
        &self, computed_parent_position: Point2<f32>, parent_size: Vector2<f32>
    ) -> Point2<f32> {
        let x = match self.docking.0 {
            Docking::Start =>
                computed_parent_position.x + self.position.x,
            Docking::End =>
                computed_parent_position.x + self.position.x + parent_size.x - self.size.x,
        };
        let y = match self.docking.1 {
            Docking::Start =>
                computed_parent_position.y + self.position.y,
            Docking::End =>
                computed_parent_position.y + self.position.y + parent_size.y - self.size.y,
        };

        Point2::new(x, y)
    }
}

#[derive(Copy, Clone)]
pub enum Docking {
    Start, End,
}

impl Docking {
    pub fn from_value(value: &Value) -> Result<(Self, Self), String> {
        let vec = value.as_vec()?;

        if vec.len() != 2 {
            return Err("Tuple is incorrect size".into())
        }

        Ok((
            Self::from_value_individual(&vec[0])?,
            Self::from_value_individual(&vec[1])?,
        ))
    }

    fn from_value_individual(value: &Value) -> Result<Self, String> {
        match value.as_string()?.as_str() {
            "start" => Ok(Docking::Start),
            "end" => Ok(Docking::End),
            _ => Err("Values must be either \"start\" or \"end\"".into())
        }
    }
}
