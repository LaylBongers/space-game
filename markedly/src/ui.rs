use nalgebra::{Vector2};
use metrohash::{MetroHashMap};

use class::{ComponentClasses};
use template::{ComponentTemplate};
use render::{Renderer};
use {Component};

pub struct Ui<R: Renderer> {
    components: MetroHashMap<ComponentId, Component<R>>,
    next_id: ComponentId,
    root_id: ComponentId,
}

impl<R: Renderer> Ui<R> {
    pub fn new(
        root: &ComponentTemplate, screen_size: Vector2<f32>, classes: &ComponentClasses<R>
    ) -> Result<Self, String> {
        let mut ui = Ui {
            components: MetroHashMap::default(),
            next_id: ComponentId(0),
            root_id: ComponentId(0),
        };

        ui.root_id = ui.load_template(root, screen_size, classes)?;

        Ok(ui)
    }

    fn load_template(
        &mut self,
        template: &ComponentTemplate, parent_size: Vector2<f32>, classes: &ComponentClasses<R>,
    ) -> Result<ComponentId, String> {
        // Load the component itself from the template
        let mut component = Component::from_template(template, parent_size, classes)?;
        let size = component.size;
        let id = self.next_id;
        self.next_id.0 += 1;

        // Also load all the children
        for child in &template.children {
            let id = self.load_template(child, size, classes)?;
            component.children.push(id);
        }

        // Add the component itself
        self.components.insert(id, component);

        Ok(id)
    }

    pub fn get(&self, id: ComponentId) -> Option<&Component<R>> {
        self.components.get(&id)
    }

    pub fn get_mut(&mut self, id: ComponentId) -> Option<&mut Component<R>> {
        self.components.get_mut(&id)
    }

    pub fn root_id(&self) -> ComponentId {
        self.root_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(pub i32);
