use pest::iterators::{Pair};

use template::parser::{Rule};

/// A value that's part of a template, to be resolved to a UI value.
#[derive(Debug, PartialEq)]
pub enum Value {
    /// A string text value.
    String(String),
    /// An integer numeric value.
    Integer(i32),
    /// A floating-point numeric value.
    Float(f32),
    /// An integer percentage value.
    Percentage(i32),
    /// A tuple of values.
    Tuple(Vec<Value>),
}

impl Value {
    pub(crate) fn parse(pair: Pair<Rule>) -> Value {
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
                    values.push(Value::parse(pair));
                }
                Value::Tuple(values)
            },
            _ => unreachable!(),
        }
    }
}
