use rlua::{Lua};
use {Error};

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

    pub fn set_model(&self) -> Result<(), Error> {
        let globals = self.lua.globals();

        let model_table = self.lua.create_table()?;
        model_table.set("build_floor_active", true)?;

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
