use pest::iterators::{Pair};
use nalgebra::{Point2, Vector2};

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

    /// Gets the floating point content of this value, or returns an error.
    pub fn as_float(&self) -> Result<f32, String> {
        match *self {
            Value::Float(value) => Ok(value),
            _ => Err("Value is not a float".into()),
        }
    }

    /// Gets the floating point content of this value, calculates a percentage floating point
    /// value, or returns an error.
    pub fn as_float_or_percentage(&self, percent_100: f32) -> Result<f32, String> {
        match *self {
            Value::Float(value) => Ok(value),
            Value::Percentage(value) => Ok((value as f32 / 100.0) * percent_100),
            _ => Err("Value is not a float or percentage".into()),
        }
    }


    /// Gets the point content of this value, or returns an error.
    pub fn as_point(&self, percent_100: Vector2<f32>) -> Result<Point2<f32>, String> {
        self.as_vector(percent_100)
            .map(|v| Point2::from_coordinates(v))
    }

    /// Gets the vector content of this value, or returns an error.
    pub fn as_vector(&self, percent_100: Vector2<f32>) -> Result<Vector2<f32>, String> {
        if let Value::Tuple(ref values) = *self {
            if values.len() == 2 {
                let x = values[0].as_float_or_percentage(percent_100.x)
                    .map_err(|e| format!("Value 1: {}", e))?;
                let y = values[1].as_float_or_percentage(percent_100.y)
                    .map_err(|e| format!("Value 2: {}", e))?;

                Ok(Vector2::new(x, y))
            } else {
                Err("Tuple is incorrect size".into())
            }
        } else {
            Err("Value is not a tuple".into())
        }
    }
}
