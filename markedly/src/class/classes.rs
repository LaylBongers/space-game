use std::collections::{HashMap};

use class::{ComponentClass};
use template::{ComponentTemplate};
use render::{Renderer};

pub struct ComponentClasses<R: Renderer> {
    factories: HashMap<String, Box<
        Fn(&ComponentTemplate) -> Result<Box<ComponentClass<R>>, String>
    >>,
}

impl<R: Renderer> ComponentClasses<R> {
    pub fn new() -> Self {
        ComponentClasses {
            factories: HashMap::new(),
        }
    }

    pub fn register<F: Fn(&ComponentTemplate) -> Result<Box<ComponentClass<R>>, String> + 'static>(
        &mut self, class: &str, factory: F
    ) {
        self.factories.insert(class.into(), Box::new(factory));
    }

    pub fn create(&self, template: &ComponentTemplate) -> Result<Box<ComponentClass<R>>, String> {
        let component_class = self.factories
            .get(&template.class)
            .ok_or(format!("Component class \"{}\" was not registered", template.class))?
            (template)?;

        Ok(component_class)
    }
}
