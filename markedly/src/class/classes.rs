use std::collections::{HashMap};

use class::{ComponentClass};
use template::{ComponentTemplate};
use {Attributes};

/// A registry of component class factories.
pub struct ComponentClasses {
    factories: HashMap<String, Box<
        Fn(&Attributes) -> Result<Box<ComponentClass>, String>
    >>,
}

impl ComponentClasses {
    /// Creates a new registry.
    pub fn new() -> Self {
        ComponentClasses {
            factories: HashMap::new(),
        }
    }

    /// Registers a component class by name.
    pub fn register<F: ComponentClassFactory>(
        &mut self, class: &str
    ) {
        self.factories.insert(class.into(), Box::new(|attributes| {
            let class = F::new(attributes)?;
            Ok(Box::new(class))
        }));
    }

    /// Creates a new boxed instance of the component class requested in the template.
    pub fn create(
        &self, template: &ComponentTemplate, attributes: &Attributes
    ) -> Result<Box<ComponentClass>, String> {
        let component_class = self.factories
            .get(&template.class)
            .ok_or(format!("Component class \"{}\" was not registered", template.class))?
            (attributes)?;

        Ok(component_class)
    }
}

/// A factory trait to allow component classes to define their factory function.
pub trait ComponentClassFactory: Sized + ComponentClass + 'static {
    /// Creates a new instance of the component class.
    fn new(attributes: &Attributes) -> Result<Self, String>;
}
