use std::collections::{HashMap};

use pest::iterators::{Pair};

use template::value::{Value};
use template::parser::{Rule};

/// A component instance in a template.
#[derive(Debug)]
pub struct ComponentInstance {
    /// The component class this is an instance of.
    pub class: String,
    /// The arguments given to this instance.
    pub arguments: HashMap<String, Value>,
    /// The children of this instance.
    pub children: Vec<ComponentInstance>,
    /// The line this component is at in the source markup.
    pub line: usize,
}

impl ComponentInstance {
    pub(crate) fn parse(pair: Pair<Rule>) -> Result<(Self, usize), String> {
        assert_eq!(pair.as_rule(), Rule::component);
        let mut indentation = 0;
        let mut class = None;
        let mut arguments = None;
        let (line, _col) = pair.clone().into_span().start_pos().line_col();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::indentation => indentation = Self::parse_indentation(pair)?,
                Rule::identifier => class = Some(pair.as_str().into()),
                Rule::arguments => arguments = Some(Self::parse_arguments(pair)?),
                _ => {}
            }
        }

        Ok((ComponentInstance {
            class: class.unwrap(),
            arguments: arguments.unwrap_or_else(|| HashMap::new()),
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

    fn parse_arguments(pair: Pair<Rule>) -> Result<HashMap<String, Value>, String> {
        assert_eq!(pair.as_rule(), Rule::arguments);

        let mut arguments: HashMap<String, Value> = HashMap::new();

        for key_value_pair in pair.into_inner() {
            assert_eq!(key_value_pair.as_rule(), Rule::key_value);

            let mut key: Option<String> = None;
            let mut value: Option<Value> = None;

            for pair in key_value_pair.clone().into_inner() {
                match pair.as_rule() {
                    Rule::identifier =>
                        key = Some(pair.as_str().into()),
                    Rule::value =>
                        value = Some(Value::parse(pair)),
                    _ => unreachable!(),
                }
            }

            // Do not allow duplicate keys
            if arguments.contains_key(key.as_ref().unwrap()) {
                let (line, _col) = key_value_pair.into_span().start_pos().line_col();
                return Err(format!("Key {} occurs more than once at line {}", key.unwrap(), line))
            }

            arguments.insert(key.unwrap(), value.unwrap());
        }

        Ok(arguments)
    }

    pub fn argument<O, F: FnOnce(&Value) -> Result<O, String>>(
        &self, key: &str, map: F, default: O
    ) -> Result<O, String> {
        self.arguments.get(key)
            .map(map)
            .unwrap_or(Ok(default))
            .map_err(|e| format!(
                "In component \"{}\" at line {}, invalid field \"{}\": {}",
                self.class, self.line, key, e
            ))
    }
}
