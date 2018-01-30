use std::collections::{HashMap};

use {Value};

/// A component in a template or style.
#[derive(Debug)]
pub struct ComponentTemplate {
    /// The component class this component has.
    pub class: String,
    /// The style class this component has.
    pub style_class: Option<String>,
    /// The attributes given to this component.
    pub attributes: HashMap<String, Value>,
    /// The children of this component.
    pub children: Vec<ComponentTemplate>,
    /// The line this component is at in the source markup.
    pub line: usize,
}
