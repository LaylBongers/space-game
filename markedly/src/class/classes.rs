use std::collections::{HashMap};

use class::{ComponentClass};
use template::{ComponentTemplate};
use render::{Renderer};

/// A registry of component class factories.
pub struct ComponentClasses<R: Renderer> {
    factories: HashMap<String, Box<
        Fn(&ComponentTemplate) -> Result<Box<ComponentClass<R>>, String>
    >>,
}

impl<R: Renderer> ComponentClasses<R> {
    /// Creates a new registry.
    pub fn new() -> Self {
        ComponentClasses {
            factories: HashMap::new(),
        }
    }

    /// Registers a component class by name.
    pub fn register<F: ComponentClassFactory<R>>(
        &mut self, class: &str
    ) {
        self.factories.insert(class.into(), Box::new(|t| {
            let class = F::new(t)?;
            Ok(Box::new(class))
        }));
    }

    /// Creates a new boxed instance of the component class requested in the template.
    pub fn create(&self, template: &ComponentTemplate) -> Result<Box<ComponentClass<R>>, String> {
        let component_class = self.factories
            .get(&template.class)
            .ok_or(format!("Component class \"{}\" was not registered", template.class))?
            (template)?;

        Ok(component_class)
    }
}

/// A factory trait to allow component classes to define their factory function.
pub trait ComponentClassFactory<R: Renderer>: Sized + ComponentClass<R> + 'static {
    // Creates a new instance of the component class.
    fn new(template: &ComponentTemplate) -> Result<Self, String>;
}
