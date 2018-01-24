use std::io::{Read};

use pest::{Parser};

use template::component::{ComponentInstance};
use template::parser::{TemplateParser, Rule};

/// A template for a user-defined UI component.
#[derive(Debug)]
pub struct Template {
    /// The root component instance of this template.
    pub root: ComponentInstance
}

impl Template {
    /// Parses a template from a reader, such as a `File`.
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, String> {
        let mut text = String::new();
        reader.read_to_string(&mut text).unwrap();
        Self::from_str(&text)
    }

    /// Parses a template from a string.
    pub fn from_str(text: &str) -> Result<Self, String> {
        // Parse and extract the template pair
        let pairs = TemplateParser::parse(Rule::template, text)
            // This gives a pretty error to our caller
            .map_err(|e| format!("{}", e))?;
        let template_pair = pairs.into_iter().next().unwrap();
        assert_eq!(template_pair.as_rule(), Rule::template);

        let mut parent_stack: Vec<ComponentInstance> = Vec::new();
        let mut has_root = false;
        let mut last_indentation = 0;
        for pair in template_pair.into_inner() {
            let (component, indentation) = ComponentInstance::parse(pair.clone())?;

            // Prevent multiple roots, or root starting at wrong indentation level
            if indentation == 0 {
                if has_root {
                    return Err("Multiple root components found".into())
                } else {
                    has_root = true;
                }
            } else {
                if !has_root {
                    return Err("First component starts at wrong indentation".into())
                }
            }

            // If we're at the same indentation level as the previous component, and not root,
            // the previous component is our sibling, not parent
            if indentation == last_indentation && indentation != 0 {
                let sibling = parent_stack.pop().expect("Internal error in parent stack");
                let mut parent = parent_stack.pop().expect("Internal error in parent stack");
                parent.children.push(sibling);
                parent_stack.push(parent);
            }

            // If we are at lower indentation level, unroll the stack to the level we need to be at
            if indentation < last_indentation {
                let unroll_amount = last_indentation - indentation + 1;
                for _ in 0..unroll_amount {
                    let sibling = parent_stack.pop().expect("Internal error in parent stack");
                    let mut parent = parent_stack.pop().expect("Internal error in parent stack");
                    parent.children.push(sibling);
                    parent_stack.push(parent);
                }
            }

            // If our indentation has increased by more than one, we need to give an error for that
            if indentation > last_indentation && indentation - last_indentation > 1 {
                let (line, _col) = pair.into_span().start_pos().line_col();
                return Err(format!("Excessive increase in indentation at line {}", line))
            }

            parent_stack.push(component);
            last_indentation = indentation;
        }

        // Unroll the stack into a final root
        let mut last_component = None;
        parent_stack.reverse();
        for mut component in parent_stack {
            if let Some(child_component) = last_component.take() {
                component.children.push(child_component);
            }
            last_component = Some(component);
        }

        Ok(Template {
            root: last_component.ok_or("No root component found")?,
        })
    }
}
