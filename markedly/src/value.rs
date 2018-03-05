use nalgebra::{Point2, Vector2};
use scripting::{ScriptRuntime};

/// A generic attribute value, will be read in by components.
#[derive(Debug, PartialEq, Clone)]
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
    /// A null value.
    Default,
    /// A script that will be evaluated by the scripting engine.
    Script(String),
}

impl Value {
    /// Gets the string content of this value, or returns an error.
    pub fn as_string(&self, runtime: &ScriptRuntime) -> Result<String, String> {
        match *self {
            Value::String(ref value) => Ok(value.clone()),
            Value::Script(ref script) => runtime.eval_string(script),
            _ => Err("Value is not a string".into()),
        }
    }

    /// Gets the integer content of this value, or returns an error.
    pub fn as_integer(&self, runtime: &ScriptRuntime) -> Result<i32, String> {
        match *self {
            Value::Integer(value) => Ok(value),
            Value::Script(ref script) => runtime.eval_integer(script),
            _ => Err("Value is not an integer".into()),
        }
    }

    /// Gets the floating point content of this value, or returns an error.
    pub fn as_float(&self, runtime: &ScriptRuntime) -> Result<f32, String> {
        match *self {
            Value::Float(value) => Ok(value),
            Value::Script(ref script) => runtime.eval_float(script),
            _ => Err("Value is not a float".into()),
        }
    }

    /// Gets the floating point content of this value, calculates a percentage floating point
    /// value, or returns an error.
    pub fn as_float_or_percentage(
        &self, percent_100: f32, runtime: &ScriptRuntime
    ) -> Result<f32, String> {
        match *self {
            Value::Float(value) => Ok(value),
            Value::Percentage(value) => Ok((value as f32 / 100.0) * percent_100),
            Value::Script(ref script) => runtime.eval_float(script),
            _ => Err("Value is not a float or percentage".into()),
        }
    }

    pub fn as_vec(&self) -> Result<&Vec<Value>, String> {
        if let Value::Tuple(ref values) = *self {
            Ok(values)
        } else {
            Err("Value is not a tuple".into())
        }
    }

    /// Gets the point content of this value, or returns an error.
    pub fn as_point(
        &self, percent_100: Vector2<f32>, runtime: &ScriptRuntime
    ) -> Result<Point2<f32>, String> {
        self.as_vector(percent_100, runtime)
            .map(|v| Point2::from_coordinates(v))
    }

    /// Gets the vector content of this value, or returns an error.
    pub fn as_vector(
        &self, percent_100: Vector2<f32>, runtime: &ScriptRuntime
    ) -> Result<Vector2<f32>, String> {
        if let Value::Tuple(ref values) = *self {
            if values.len() == 2 {
                let x = values[0].as_float_or_percentage(percent_100.x, runtime)
                    .map_err(|e| format!("Value 1: {}", e))?;
                let y = values[1].as_float_or_percentage(percent_100.y, runtime)
                    .map_err(|e| format!("Value 2: {}", e))?;

                Ok(Vector2::new(x, y))
            } else {
                Err("Tuple is incorrect size".into())
            }
        } else {
            Err("Value is not a tuple".into())
        }
    }

    /// Gets the color content of this value, or returns an error.
    pub fn as_color(&self, runtime: &ScriptRuntime) -> Result<Color, String> {
        if let Value::Tuple(ref values) = *self {
            let has_alpha = values.len() == 4;
            if values.len() == 3 || has_alpha {
                let red = values[0].as_integer(runtime)
                    .map_err(|e| format!("Value 1: {}", e))?;
                let green = values[1].as_integer(runtime)
                    .map_err(|e| format!("Value 2: {}", e))?;
                let blue = values[2].as_integer(runtime)
                    .map_err(|e| format!("Value 3: {}", e))?;
                let alpha = if has_alpha {
                    let alpha = values[3].as_float(runtime)
                        .map_err(|e| format!("Value 4: {}", e))?;
                    range_f(alpha, "Value 4", 0.0, 1.0)?;
                    (255.0 * alpha).round() as u8
                } else {
                    255
                };

                range_i(red, "Value 1", 0, 255)?;
                range_i(green, "Value 2", 0, 255)?;
                range_i(blue, "Value 3", 0, 255)?;

                Ok(Color::with_alpha_u8(red as u8, green as u8, blue as u8, alpha))
            } else {
                Err("Tuple is incorrect size".into())
            }
        } else {
            Err("Value is not a tuple".into())
        }
    }
}

fn range_i(value: i32, err_id: &str, min: i32, max: i32) -> Result<(), String> {
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(format!("{}: Out of range, valid range is {} to {}", err_id, min, max))
    }
}

fn range_f(value: f32, err_id: &str, min: f32, max: f32) -> Result<(), String> {
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(format!("{}: Out of range, valid range is {} to {}", err_id, min, max))
    }
}

/// Re-export of palette's color for convenience so you don't have to add palette to your own
/// crate unless you need more complex color functionality.
pub type Color = ::palette::pixel::Srgb;
