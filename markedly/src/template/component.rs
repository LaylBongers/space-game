use std::collections::{HashMap};

use {Value};

/// A component instance in a template.
#[derive(Debug)]
pub struct ComponentTemplate {
    /// The component class this coomponent has.
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

impl ComponentTemplate {
    pub fn attribute<O, F: FnOnce(&Value) -> Result<O, String>>(
        &self, key: &str, map: F, default: O
    ) -> Result<O, String> {
        self.attributes.get(key)
            .map(map)
            .unwrap_or(Ok(default))
            .map_err(|e| format!(
                "In component \"{}\" at line {}, invalid field \"{}\": {}",
                self.class, self.line, key, e
            ))
    }

    pub fn attribute_optional<O, F: FnOnce(&Value) -> Result<O, String>>(
        &self, key: &str, map: F,
    ) -> Result<Option<O>, String> {
        self.attributes.get(key)
            .map(|value| {
                if *value == Value::Null {
                    Ok(None)
                } else {
                    map(value).map(|v| Some(v))
                }
            })
            .unwrap_or(Ok(None))
            .map_err(|e| format!(
                "In component \"{}\" at line {}, invalid field \"{}\": {}",
                self.class, self.line, key, e
            ))
    }
}
