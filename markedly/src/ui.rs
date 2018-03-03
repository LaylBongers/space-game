use nalgebra::{Vector2};
use metrohash::{MetroHashMap};

use class::{ComponentClasses};
use scripting::{ScriptRuntime};
use template::{Template, ComponentTemplate, Style};
use {Component, ComponentEvents};

/// A self-contained UI.
pub struct Ui {
    components: MetroHashMap<ComponentId, Component>,
    next_id: ComponentId,
    root_id: ComponentId,
}

impl Ui {
    /// Creates a new UI from a root template.
    pub fn new(
        root: &Template, style: &Style,
        screen_size: Vector2<f32>, classes: &ComponentClasses,
        runtime: &ScriptRuntime,
    ) -> Result<(Self, ComponentEvents), String> {
        let mut ui = Ui {
            components: MetroHashMap::default(),
            next_id: ComponentId(0),
            root_id: ComponentId(0),
        };

        let events = ComponentEvents::new();
        ui.root_id = ui.load_component(&root.root, style, screen_size, classes, &events, runtime)?;

        Ok((ui, events))
    }

    /// Gets a component from its ID.
    pub fn get(&self, id: ComponentId) -> Option<&Component> {
        self.components.get(&id)
    }

    /// Gets a component as mutable from its ID.
    pub fn get_mut(&mut self, id: ComponentId) -> Option<&mut Component> {
        self.components.get_mut(&id)
    }

    /// Gets the root component's ID.
    pub fn root_id(&self) -> ComponentId {
        self.root_id
    }

    /// Inserts a template into the ui as a child of the first found component that has the given
    /// style class.
    pub fn insert_template(
        &mut self,
        template: &Template, style: &Style,
        style_class: &str, classes: &ComponentClasses,
        runtime: &ScriptRuntime,
    ) -> Result<ComponentEvents, String> {
        // Find the first component that has a style class matching what we were asked for
        let mut found_parent_id = None;
        for (key, component) in &self.components {
            if let Some(ref component_style_class) = component.style_class {
                if component_style_class == style_class {
                    found_parent_id = Some(*key);
                }
            }
        }

        // Make sure we found something and retrieve some basic data we need
        let parent_id = found_parent_id
            .ok_or(format!("Unable to find component with style class {}", style_class))?;
        let size = self.get(parent_id).unwrap().size;

        // Recursively add the template
        let events = ComponentEvents::new();
        let id = self.load_component(&template.root, style, size, classes, &events, runtime)?;

        // Add the component tree we just added to the children of the component we had found
        self.get_mut(parent_id).unwrap().children.push(id);

        Ok(events)
    }

    fn load_component(
        &mut self,
        template: &ComponentTemplate, style: &Style,
        parent_size: Vector2<f32>, classes: &ComponentClasses,
        events: &ComponentEvents, runtime: &ScriptRuntime
    ) -> Result<ComponentId, String> {
        // Load the component itself from the template
        let mut component = Component::from_template(
            template, style, parent_size, classes, events, runtime,
        )?;
        let size = component.size;
        let id = self.next_id;
        self.next_id.0 += 1;

        // Also load all the children
        for child in &template.children {
            let id = self.load_component(child, style, size, classes, events, runtime)?;
            component.children.push(id);
        }

        // Add the component itself
        self.components.insert(id, component);

        Ok(id)
    }
}

/// An id pointing to a component in a UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(pub i32);
