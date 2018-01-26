use std::collections::{HashMap};

use pest::iterators::{Pair};

use {Value};
use template::parser::{Rule};

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
    pub(crate) fn parse(pair: Pair<Rule>) -> Result<(Self, usize), String> {
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
