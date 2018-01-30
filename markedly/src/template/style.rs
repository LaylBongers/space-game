use std::io::{Read};

use template::{ComponentTemplate};

#[derive(Debug)]
pub struct Style {
    pub components: Vec<ComponentTemplate>,
}

impl Style {
    /// Parses a style from a reader, such as a `File`.
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, String> {
        let mut text = String::new();
        reader.read_to_string(&mut text).unwrap();
        Self::from_str(&text)
    }

    /// Parses a style from a string.
    pub fn from_str(_text: &str) -> Result<Self, String> {
        Ok(Style {
            components: Vec::new(),
        })
    }
}
