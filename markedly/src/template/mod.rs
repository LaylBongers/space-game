//! Templates parsed in from markup

mod component;
mod parse;
mod style;
mod template;

pub use self::component::{ComponentTemplate, TemplateAttribute};
pub use self::style::{Style};
pub use self::template::{Template};
