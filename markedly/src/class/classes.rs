use std::collections::{HashMap};

use class::{ComponentClass};
use template::{ComponentTemplate};

/// A registry of component class factories.
pub struct ComponentClasses {
    factories: HashMap<String, Box<
        Fn(&ComponentTemplate) -> Result<Box<ComponentClass>, String>
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
        self.factories.insert(class.into(), Box::new(|t| {
            let class = F::new(t)?;
            Ok(Box::new(class))
        }));
    }

    /// Creates a new boxed instance of the component class requested in the template.
    pub fn create(&self, template: &ComponentTemplate) -> Result<Box<ComponentClass>, String> {
        let component_class = self.factories
            .get(&template.class)
            .ok_or(format!("Component class \"{}\" was not registered", template.class))?
            (template)?;

        Ok(component_class)
    }
}

/// A factory trait to allow component classes to define their factory function.
pub trait ComponentClassFactory: Sized + ComponentClass + 'static {
    /// Creates a new instance of the component class.
    fn new(template: &ComponentTemplate) -> Result<Self, String>;
}
