use nalgebra::{Point2, Vector2};

use class::{ComponentClass};
use scripting::{ScriptRuntime};
use template::{ComponentTemplate, Style, Value};
use {ComponentId, ComponentEvents, Attributes, Error, UiContext};

/// Core attributes all components share.
pub struct ComponentAttributes {
    pub position: Point2<f32>,
    pub size: Vector2<f32>,
    pub docking: (Docking, Docking),
}

impl ComponentAttributes {
    pub fn load(
        parent_size: Vector2<f32>, attributes: &Attributes, runtime: &ScriptRuntime
    ) -> Result<Self, Error> {
        Ok(ComponentAttributes {
            position: attributes.attribute(
                "position", |v| v.as_point(parent_size, runtime), Point2::new(0.0, 0.0)
            )?,
            size: attributes.attribute(
                "size", |v| v.as_vector(parent_size, runtime), parent_size
            )?,
            docking: attributes.attribute(
                "docking", |v| Docking::from_value(v, runtime), (Docking::Start, Docking::Start)
            )?,
        })
    }
}

/// A component generated from a template, active in a UI.
pub struct Component {
    pub(crate) class: Box<ComponentClass>,
    pub(crate) style_class: Option<String>,
    pub(crate) events: ComponentEvents,
    template: ComponentTemplate,

    pub(crate) children: Vec<ComponentId>,
    pub(crate) attributes: ComponentAttributes,
}

impl Component {
    pub(crate) fn from_template(
        template: &ComponentTemplate, events: &ComponentEvents,
        style: &Style,
        parent_size: Vector2<f32>,
        context: &UiContext,
    ) -> Result<Self, Error> {
        let runtime = &context.runtime;
        let attributes = Attributes::resolve(template, style, context)?;

        let class = context.classes.create(template, &attributes, runtime)?;
        let component_attributes = ComponentAttributes::load(parent_size, &attributes, runtime)?;

        Ok(Component {
            class,
            style_class: template.style_class.clone(),
            events: events.clone(),
            // TODO: This seems very expensive to store, find alternatives
            template: template.clone(),

            children: Vec::new(),
            attributes: component_attributes,
        })
    }

    pub(crate) fn update_attributes(
        &mut self, style: &Style, context: &UiContext
    ) -> Result<(), Error> {
        // TODO: Update own attributes

        let runtime = &context.runtime;
        let attributes = Attributes::resolve(&self.template, style, context)?;
        self.class.update_attributes(&attributes, runtime)?;

        Ok(())
    }

    pub(crate) fn compute_position(
        &self, computed_parent_position: Point2<f32>, parent_size: Vector2<f32>
    ) -> Point2<f32> {
        let x = match self.attributes.docking.0 {
            Docking::Start =>
                computed_parent_position.x + self.attributes.position.x,
            Docking::End =>
                computed_parent_position.x + self.attributes.position.x +
                    parent_size.x - self.attributes.size.x,
        };
        let y = match self.attributes.docking.1 {
            Docking::Start =>
                computed_parent_position.y + self.attributes.position.y,
            Docking::End =>
                computed_parent_position.y + self.attributes.position.y +
                    parent_size.y - self.attributes.size.y,
        };

        Point2::new(x, y)
    }
}

#[derive(Copy, Clone)]
pub enum Docking {
    Start, End,
}

impl Docking {
    pub fn from_value(value: &Value, runtime: &ScriptRuntime) -> Result<(Self, Self), Error> {
        let vec = value.as_vec()?;

        if vec.len() != 2 {
            return Err("Tuple is incorrect size".into())
        }

        Ok((
            Self::from_value_individual(&vec[0], runtime)?,
            Self::from_value_individual(&vec[1], runtime)?,
        ))
    }

    fn from_value_individual(value: &Value, runtime: &ScriptRuntime) -> Result<Self, Error> {
        match value.as_string(runtime)?.as_str() {
            "start" => Ok(Docking::Start),
            "end" => Ok(Docking::End),
            _ => Err("Values must be either \"start\" or \"end\"".into())
        }
    }
}
