use std::io::{Read};
use std::collections::{HashMap};

use pest::{Parser};
use pest::iterators::{Pair};

use {Value};
use template::parser::{TemplateParser, Rule};


/// A component instance in a template.
#[derive(Debug)]
pub struct ComponentTemplate {
    /// The component class this is an instance of.
    pub class: String,
    /// The attributes given to this instance.
    pub attributes: HashMap<String, Value>,
    /// The children of this instance.
    pub children: Vec<ComponentTemplate>,
    /// The line this component is at in the source markup.
    pub line: usize,
}

impl ComponentTemplate {
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

        let mut parent_stack: Vec<ComponentTemplate> = Vec::new();
        let mut has_root = false;
        let mut last_indentation = 0;
        for pair in template_pair.into_inner() {
            let (component, indentation) = Self::parse_component(pair.clone())?;

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

        last_component.ok_or("No root component found".into())
    }

    fn parse_component(pair: Pair<Rule>) -> Result<(Self, usize), String> {
        assert_eq!(pair.as_rule(), Rule::component);
        let mut indentation = 0;
        let mut class = None;
        let mut attributes = None;
        let (line, _col) = pair.clone().into_span().start_pos().line_col();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::indentation => indentation = Self::parse_indentation(pair)?,
                Rule::identifier => class = Some(pair.as_str().into()),
                Rule::attributes => attributes = Some(Self::parse_attributes(pair)?),
                _ => {}
            }
        }

        Ok((ComponentTemplate {
            class: class.unwrap(),
            attributes: attributes.unwrap_or_else(|| HashMap::new()),
            children: Vec::new(),
            line,
        }, indentation))
    }

    fn parse_indentation(pair: Pair<Rule>) -> Result<usize, String> {
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

        Ok(spacing/4)
    }

    fn parse_attributes(pair: Pair<Rule>) -> Result<HashMap<String, Value>, String> {
        assert_eq!(pair.as_rule(), Rule::attributes);

        let mut attributes: HashMap<String, Value> = HashMap::new();

        for key_value_pair in pair.into_inner() {
            assert_eq!(key_value_pair.as_rule(), Rule::key_value);

            let mut key: Option<String> = None;
            let mut value: Option<Value> = None;

            for pair in key_value_pair.clone().into_inner() {
                match pair.as_rule() {
                    Rule::identifier =>
                        key = Some(pair.as_str().into()),
                    Rule::value =>
                        value = Some(Self::parse_value(pair)),
                    _ => unreachable!(),
                }
            }

            // Do not allow duplicate keys
            if attributes.contains_key(key.as_ref().unwrap()) {
                let (line, _col) = key_value_pair.into_span().start_pos().line_col();
                return Err(format!("Key {} occurs more than once at line {}", key.unwrap(), line))
            }

            attributes.insert(key.unwrap(), value.unwrap());
        }

        Ok(attributes)
    }

    fn parse_value(pair: Pair<Rule>) -> Value {
        assert_eq!(pair.as_rule(), Rule::value);
        let pair = pair.into_inner().next().unwrap();

        let pair_str = pair.as_str();
        match pair.as_rule() {
            Rule::string =>
                Value::String(pair_str[1..pair_str.len()-1].into()),
            Rule::percentage =>
                Value::Percentage(pair_str[0..pair_str.len()-1].parse().unwrap()),
            Rule::integer =>
                Value::Integer(pair_str.parse().unwrap()),
            Rule::float =>
                Value::Float(pair_str.parse().unwrap()),
            Rule::tuple => {
                let mut values = Vec::new();
                for pair in pair.into_inner() {
                    values.push(Self::parse_value(pair));
                }
                Value::Tuple(values)
            },
            Rule::null =>
                Value::Null,
            _ => unreachable!(),
        }
    }

    pub fn attribute<O, F: FnOnce(&Value) -> Result<O, String>>(
        &self, key: &str, map: F, default: O
    ) -> Result<O, String> {
        self.attributes.get(key)
            .map(map)
            .unwrap_or(Ok(default))
            .map_err(|e| format!(
                "In component \"{}\" at line {}, invalid field \"{}\": {}",
                self.class, self.line, key, e
            ))
    }

    pub fn attribute_optional<O, F: FnOnce(&Value) -> Result<O, String>>(
        &self, key: &str, map: F,
    ) -> Result<Option<O>, String> {
        self.attributes.get(key)
            .map(|value| {
                if *value == Value::Null {
                    Ok(None)
                } else {
                    map(value).map(|v| Some(v))
                }
            })
            .unwrap_or(Ok(None))
            .map_err(|e| format!(
                "In component \"{}\" at line {}, invalid field \"{}\": {}",
                self.class, self.line, key, e
            ))
    }
}
