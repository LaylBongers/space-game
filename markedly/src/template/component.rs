use pest::iterators::{Pair};

use template::value::{Value};
use template::parser::{Rule};

/// A component instance in a template.
#[derive(Debug, PartialEq)]
pub struct ComponentInstance {
    /// The component this is an instance of.
    pub component: String,
    /// The arguments given to this instance.
    pub arguments: Vec<(String, Value)>,
    /// The children of this instance.
    pub children: Vec<ComponentInstance>,
}

impl ComponentInstance {
    pub(crate) fn parse(pair: Pair<Rule>) -> Result<(Self, usize), String> {
        assert_eq!(pair.as_rule(), Rule::component);
        let mut indentation = 0;
        let mut component = None;
        let mut arguments = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::indentation => indentation = Self::parse_indentation(pair)?,
                Rule::identifier => component = Some(pair.as_str().into()),
                Rule::arguments => arguments = Some(Self::parse_arguments(pair)),
                _ => {}
            }
        }

        Ok((ComponentInstance {
            component: component.unwrap(),
            arguments: arguments.unwrap_or_else(|| Vec::new()),
            children: Vec::new(),
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

    fn parse_arguments(pair: Pair<Rule>) -> Vec<(String, Value)> {
        assert_eq!(pair.as_rule(), Rule::arguments);

        let mut arguments = Vec::new();

        for key_value_pair in pair.into_inner() {
            assert_eq!(key_value_pair.as_rule(), Rule::key_value);

            let mut key: Option<String> = None;
            let mut value: Option<Value> = None;

            for pair in key_value_pair.into_inner() {
                match pair.as_rule() {
                    Rule::identifier =>
                        key = Some(pair.as_str().into()),
                    Rule::string | Rule::integer | Rule::float =>
                        value = Some(Value::parse(pair)),
                    _ => unreachable!(),
                }
            }

            arguments.push((key.unwrap(), value.unwrap()));
        }

        arguments
    }
}
