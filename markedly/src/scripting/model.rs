use std::collections::{HashMap};
use rlua::{Lua, Table};
use {Error};

pub struct Model {
    values: HashMap<String, ModelValue>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            values: HashMap::new(),
        }
    }

    pub(crate) fn to_lua_table<'l>(&self, lua: &'l Lua) -> Result<Table<'l>, Error> {
        let model_table = lua.create_table()?;

        for (key, value) in &self.values {
            match *value {
                ModelValue::Bool(value) => model_table.set(key.as_str(), value)?,
                ModelValue::String(ref value) => model_table.set(key.as_str(), value.as_str())?,
            }
        }

        Ok(model_table)
    }

    pub fn set<V: Into<ModelValue>>(&mut self, key: &str, value: V) {
        self.values.insert(key.into(), value.into());
    }
}

pub enum ModelValue {
    Bool(bool),
    String(String),
}

impl From<bool> for ModelValue {
    fn from(value: bool) -> Self {
        ModelValue::Bool(value)
    }
}

impl From<String> for ModelValue {
    fn from(value: String) -> Self {
        ModelValue::String(value)
    }
}
