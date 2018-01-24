use std::io::{Read};

use pest::{Parser};
use pest::iterators::{Pair};

#[derive(Parser)]
#[grammar = "template/template.pest"]
struct TemplateParser;

#[derive(Debug)]
pub struct Template {
    root: TemplateComponent
}

impl Template {
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self, String> {
        let mut text = String::new();
        reader.read_to_string(&mut text).unwrap();
        Self::from_str(&text)
    }

    pub fn from_str(text: &str) -> Result<Self, String> {
        // Parse and extract the template pair
        let pairs = TemplateParser::parse(Rule::template, text)
            // This gives a pretty error to our caller
            .map_err(|e| format!("{}", e))?;
        let template_pair = pairs.into_iter().next().unwrap();
        assert_eq!(template_pair.as_rule(), Rule::template);

        let mut parent_stack: Vec<TemplateComponent> = Vec::new();
        let mut has_root = false;
        let mut last_indentation = 0;
        for pair in template_pair.into_inner() {
            let (component, indentation) = TemplateComponent::parse(pair.clone())?;

            // Prevent multiple roots, or root starting at wrong indentation level
            if indentation == 0 {
                if has_root {
                    return Err("Multiple root components found".to_string())
                } else {
                    has_root = true;
                }
            } else {
                if !has_root {
                    return Err("First component starts at wrong indentation".to_string())
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

#[derive(Debug)]
pub struct TemplateComponent {
    pub class: String,
    pub args: Vec<(String, TemplateValue)>,
    pub children: Vec<TemplateComponent>,
}

impl TemplateComponent {
    fn parse(pair: Pair<Rule>) -> Result<(Self, usize), String> {
        assert_eq!(pair.as_rule(), Rule::component);
        let mut indentation = 0;
        let mut class = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::indentation => {
                    // Count the spacing, including tabs
                    let mut spacing = 0;
                    for c in pair.as_str().chars() {
                        match c {
                            ' ' => spacing += 1,
                            '\t' => spacing += 4,
                            _ => unreachable!(),
                        }
                    }

                    // Fail indentation that isn't divisible by 4
                    if spacing % 4 != 0 {
                        let (line, _col) = pair.into_span().start_pos().line_col();
                        return Err(format!("Bad amount of indentation spacing, must be divisible by 4, at line {}", line))
                    }

                    indentation = spacing/4;
                },
                Rule::identifier => class = Some(pair.as_str().to_string()),
                _ => {}
            }
        }

        Ok((TemplateComponent {
            class: class.unwrap(),
            args: Vec::new(),
            children: Vec::new(),
        }, indentation))
    }
}

#[derive(Debug)]
pub enum TemplateValue {
    String(String),
}

#[cfg(test)]
mod test {
    use super::{Template};

    #[test]
    fn it_parses_single_root() {
        let result = Template::from_str("root\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().root.class, "root");
    }

    #[test]
    fn it_fails_two_roots() {
        let result = Template::from_str("root\nroot2\n");

        println!("Result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn it_parses_root_with_child() {
        let result = Template::from_str("root\n    child\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 1, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child");
    }

    #[test]
    fn it_parses_root_with_nested_children() {
        let result = Template::from_str("root\n    child\n        nested_child\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 1, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child");
        assert_eq!(root.children[0].children.len(), 1, "Incorrect children length on child");
        assert_eq!(root.children[0].children[0].class, "nested_child");
    }

    #[test]
    fn it_parses_root_with_two_children() {
        let result = Template::from_str("root\n    child1\n    child2\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 2, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child1");
        assert_eq!(root.children[1].class, "child2");
    }

    #[test]
    fn it_parses_varied_children_depth() {
        let result = Template::from_str("root\n    child1\n        nested_child\n    child2\n");

        println!("Result: {:?}", result);
        assert!(result.is_ok());
        let root = result.unwrap().root;
        assert_eq!(root.class, "root");
        assert_eq!(root.children.len(), 2, "Incorrect children length on root");
        assert_eq!(root.children[0].class, "child1");
        assert_eq!(root.children[1].class, "child2");
        assert_eq!(root.children[0].children.len(), 1, "Incorrect children length on child1");
        assert_eq!(root.children[0].children[0].class, "nested_child");
    }

    #[test]
    fn it_fails_excessive_indentation() {
        let result = Template::from_str("root\n        excessive_child1\n");

        println!("Result: {:?}", result);
        assert!(result.is_err());
    }

    #[test]
    fn it_fails_non_4_indentation() {
        let result1 = Template::from_str("root\n  bad_child\n");
        let result2 = Template::from_str("root\n     bad_child\n");

        println!("Result1: {:?}", result1);
        println!("Result2: {:?}", result2);
        assert!(result1.is_err());
        assert!(result2.is_err());
    }
}
