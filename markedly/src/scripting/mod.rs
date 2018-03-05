use std::collections::{HashMap};
use rlua::{Lua};
use {Error};

pub type Model = HashMap<String, ScriptValue>;

pub enum ScriptValue {
    Bool(bool),
    String(String),
}

pub struct ScriptRuntime {
    lua: Lua,
}

impl ScriptRuntime {
    pub fn new() -> Self {
        let lua = Lua::new();

        ScriptRuntime {
            lua,
        }
    }

    pub fn set_model(&self, model: &Model) -> Result<(), Error> {
        let globals = self.lua.globals();

        let model_table = self.lua.create_table()?;
        for (key, value) in model {
            match *value {
                ScriptValue::Bool(value) => model_table.set(key.as_str(), value)?,
                ScriptValue::String(ref value) => model_table.set(key.as_str(), value.as_str())?,
            }
        }

        globals.set("model", model_table)?;

        Ok(())
    }

    pub fn eval_bool(&self, source: &str) -> Result<bool, Error> {
        let value = self.lua.eval(source, None)?;
        Ok(value)
    }

    pub fn eval_integer(&self, source: &str) -> Result<i32, Error> {
        let value = self.lua.eval(source, None)?;
        Ok(value)
    }

    pub fn eval_float(&self, source: &str) -> Result<f32, Error> {
        let value = self.lua.eval(source, None)?;
        Ok(value)
    }

    pub fn eval_string(&self, source: &str) -> Result<String, Error> {
        let value = self.lua.eval(source, None)?;
        Ok(value)
    }
}
