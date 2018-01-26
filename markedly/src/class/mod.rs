mod container;
mod button;

pub use self::container::{ContainerClass};
pub use self::button::{ButtonClass};

use std::collections::{HashMap};
use nalgebra::{Point2, Vector2};

use {Renderer};

pub struct ComponentClasses<R: Renderer> {
    factories: HashMap<String, Box< Fn() -> Box<ComponentClass<R>> >>,
}

impl<R: Renderer> ComponentClasses<R> {
    pub fn new() -> Self {
        ComponentClasses {
            factories: HashMap::new(),
        }
    }

    pub fn register<F: Fn() -> Box<ComponentClass<R>> + 'static>(
        &mut self, class: &str, factory: F
    ) {
        self.factories.insert(class.into(), Box::new(factory));
    }

    pub fn create(&self, class: &str) -> Result<Box<ComponentClass<R>>, String> {
        let component_class = self.factories
            .get(class)
            .ok_or(format!("Component class \"{}\" was not registered", class))?
            ();

        Ok(component_class)
    }
}

pub trait ComponentClass<R: Renderer> {
    fn render(
        &self, renderer: &R, context: &mut R::Context, position: Point2<f32>, size: Vector2<f32>
    ) -> Result<(), R::Error>;
}
