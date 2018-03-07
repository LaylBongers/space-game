//! Component classes that define functionality and appearance.

mod container;
mod classes;
mod button;

pub use self::container::{ContainerClass};
pub use self::classes::{ComponentClasses, ComponentClassFactory};
pub use self::button::{ButtonClass};

use render::{Renderer};
use scripting::{ScriptRuntime};
use {ComponentEvents, ComponentAttributes, Error, Attributes, ComponentId};

/// The class of a component, defines specific appearance and functionality in response to user
/// input.
pub trait ComponentClass {
    fn update_attributes(
        &mut self, attributes: &Attributes, runtime: &ScriptRuntime,
    ) -> Result<(), Error>;

    /// Renders the component.
    fn render(
        &self, id: ComponentId, attributes: &ComponentAttributes, renderer: &mut Renderer,
    ) -> Result<(), Error>;

    /// Returns if this component class captures cursor events or not. Does not affect children.
    fn is_capturing_cursor(&self) -> bool { false }

    /// Called when the cursor starts hovering over this component.
    /// Returns if the component should be marked for render update.
    fn hover_start_event(&mut self, _sender: &ComponentEvents) -> bool { false }

    /// Called when the cursor stops hovering over this component.
    /// Returns if the component should be marked for render update.
    fn hover_end_event(&mut self, _sender: &ComponentEvents) -> bool { false }

    /// Called when the component is clicked or tapped.
    fn pressed_event(&mut self, _sender: &ComponentEvents) {}
}
