use rlua::{Lua};

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

    pub fn set_model(&self) -> Result<(), String> {
        let globals = self.lua.globals();

        let model_table = self.lua.create_table()
            .map_err(|e| format!("{}", e))?;
        model_table.set("build_floor_active", true)
            .map_err(|e| format!("{}", e))?;

        globals.set("model", model_table)
            .map_err(|e| format!("{}", e))?;

        Ok(())
    }

    pub fn eval_bool(&self, source: &str) -> Result<bool, String> {
        let value = self.lua.eval(source, None)
            .map_err(|e| format!("{}", e))?;
        Ok(value)
    }

    pub fn eval_integer(&self, source: &str) -> Result<i32, String> {
        let value = self.lua.eval(source, None)
            .map_err(|e| format!("{}", e))?;
        Ok(value)
    }

    pub fn eval_float(&self, source: &str) -> Result<f32, String> {
        let value = self.lua.eval(source, None)
            .map_err(|e| format!("{}", e))?;
        Ok(value)
    }

    pub fn eval_string(&self, source: &str) -> Result<String, String> {
        let value = self.lua.eval(source, None)
            .map_err(|e| format!("{}", e))?;
        Ok(value)
    }
}
