use scripting::{ScriptRuntime};
use template::{Value};
use {Error};

/// A component in a template or style.
#[derive(Clone, Debug)]
pub struct ComponentTemplate {
    /// The component class this component has.
    pub class: String,
    /// The style class this component has.
    pub style_class: Option<String>,
    /// The attributes given to this component.
    pub attributes: Vec<TemplateAttribute>,
    /// The children of this component.
    pub children: Vec<ComponentTemplate>,
    /// The line this component is at in the source markup.
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct TemplateAttribute {
    pub key: String,
    pub value: Value,
    pub script_conditional: Option<String>,
}

impl TemplateAttribute {
    pub fn check_conditional(&self, runtime: &ScriptRuntime) -> Result<bool, Error> {
        if let Some(ref script) = self.script_conditional {
            runtime.eval_bool(script)
        } else {
            Ok(true)
        }
    }
}
