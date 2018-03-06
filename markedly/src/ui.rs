use std::collections::{HashMap};

use nalgebra::{Vector2};
use metrohash::{MetroHashMap};

use scripting::{Model};
use template::{Template, ComponentTemplate};
use {Component, ComponentEvents, Error, UiContext};

/// A self-contained UI.
pub struct Ui {
    components: MetroHashMap<ComponentId, Component>,
    next_id: ComponentId,
    root_id: ComponentId,
    models: MetroHashMap<ComponentId, ComponentEvents>,
}

impl Ui {
    /// Creates a new UI from a root template.
    pub fn new(root: &Template, context: &UiContext) -> Result<(Self, ComponentEvents), Error> {
        let mut ui = Ui {
            components: MetroHashMap::default(),
            next_id: ComponentId(0),
            root_id: ComponentId(0),
            models: MetroHashMap::default(),
        };

        let events = ComponentEvents::new(HashMap::new());
        ui.root_id = ui.load_component(&root.root, &events, context.screen_size, context)?;

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

    pub fn update_ui_from_models(&mut self, context: &UiContext) -> Result<(), Error> {
        for (key, value) in &self.models {
            if value.model_changed() {
                // The model has changed, update all components' data
                // Reloading everything isn't very efficient, it should be changed later to
                // detect which model values components have been bound to and only update the
                // relevant ones
                context.runtime.set_model(&value.model.borrow().0)?;

                Self::update_component_recursive(
                    &mut self.components, *key, &self.models, context
                )?;

                value.clear_changed();
            }
        }

        Ok(())
    }

    fn update_component_recursive(
        components: &mut MetroHashMap<ComponentId, Component>, key: ComponentId,
        models: &MetroHashMap<ComponentId, ComponentEvents>, context: &UiContext,
    ) -> Result<(), Error> {
        for child_i in 0..components.get(&key).unwrap().children.len() {
            let child_id = components.get(&key).unwrap().children[child_i];

            // Do not go deeper if we're at an inserted template's root
            if !models.contains_key(&child_id) {
                Self::update_component_recursive(components, child_id, models, context)?;
            }
        }

        components.get_mut(&key).unwrap().update_attributes(context)?;

        Ok(())
    }

    /// Inserts a template into the ui as a child of the first found component that has the given
    /// style class.
    pub fn insert_template(
        &mut self,
        template: &Template, model: Option<Model>,
        style_class: &str,
        context: &UiContext,
    ) -> Result<ComponentEvents, Error> {
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
        let size = self.get(parent_id).unwrap().attributes.size;

        // Prepare the scripting engine with the model data
        let model = model.unwrap_or(HashMap::new());
        context.runtime.set_model(&model)?;

        // Recursively add the template
        let events = ComponentEvents::new(model);
        let id = self.load_component(&template.root, &events, size, context)?;

        // Add the component tree we just added to the children of the component we had found
        self.get_mut(parent_id).unwrap().children.push(id);

        // Store an id-model combination so we can update the tree if the model changes
        self.models.insert(id, events.clone());

        Ok(events)
    }

    fn load_component(
        &mut self,
        template: &ComponentTemplate, events: &ComponentEvents,
        parent_size: Vector2<f32>,
        context: &UiContext,
    ) -> Result<ComponentId, Error> {
        // Load the component itself from the template
        let mut component = Component::from_template(
            template, events, parent_size, context,
        )?;
        let size = component.attributes.size;
        let id = self.next_id;
        self.next_id.0 += 1;

        // Also load all the children
        for child in &template.children {
            let id = self.load_component(child, events, size, context)?;
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
