use std::collections::{HashMap};

use template::{ComponentTemplate, Style};
use {Value};

pub struct Attributes {
    attributes: HashMap<String, Value>,
}

impl Attributes {
    /// Resolves the final attributes of the current component from its template and the style.
    pub fn resolve(template: &ComponentTemplate, style: &Style) -> Self {
        let mut attributes = HashMap::new();

        // Add any styles from the stylesheet
        for component in &style.components {
            if component.class == template.class {
                for (key, value) in &component.attributes {
                    attributes.insert(key.clone(), value.clone());
                }
            }
        }

        // Overwrite any style resolved attributes with this component's set attributes
        for (key, value) in &template.attributes {
            attributes.insert(key.clone(), value.clone());
        }

        Attributes {
            attributes,
        }
    }

    pub fn attribute<O, F: FnOnce(&Value) -> Result<O, String>>(
        &self, key: &str, map: F, default: O
    ) -> Result<O, String> {
        self.attributes.get(key)
            .map(map)
            .unwrap_or(Ok(default))
            .map_err(|e| format!(
                // TODO: Report source of error
                "Invalid field \"{}\": {}",
                key, e
            ))
    }

    pub fn attribute_optional<O, F: FnOnce(&Value) -> Result<O, String>>(
        &self, key: &str, map: F,
    ) -> Result<Option<O>, String> {
        self.attributes.get(key)
            .map(|value| {
                if *value == Value::Default {
                    Ok(None)
                } else {
                    map(value).map(|v| Some(v))
                }
            })
            .unwrap_or(Ok(None))
            .map_err(|e| format!(
                // TODO: Report source of error
                "Invalid field \"{}\": {}",
                key, e
            ))
    }
}
