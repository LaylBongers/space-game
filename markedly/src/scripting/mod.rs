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

    pub fn eval_string(&self, source: &str) -> Result<String, String> {
        let value = self.lua.eval::<String>(source, None)
            .map_err(|e| format!("{}", e))?;
        Ok(value)
    }
}
