use nalgebra::{Point2, Vector2};

/// A generic attribute value, will be read in by components.
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
    /// A null value.
    Null,
}

impl Value {
    /// Gets the string content of this value, or returns an error.
    pub fn as_string(&self) -> Result<String, String> {
        match *self {
            Value::String(ref value) => Ok(value.clone()),
            _ => Err("Value is not a string".into()),
        }
    }

    /// Gets the integer content of this value, or returns an error.
    pub fn as_integer(&self) -> Result<i32, String> {
        match *self {
            Value::Integer(value) => Ok(value),
            _ => Err("Value is not an integer".into()),
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

    /// Gets the color content of this value, or returns an error.
    pub fn as_color(&self) -> Result<Color, String> {
        if let Value::Tuple(ref values) = *self {
            let has_alpha = values.len() == 4;
            if values.len() == 3 || has_alpha {
                let red = values[0].as_integer()
                    .map_err(|e| format!("Value 1: {}", e))?;
                let green = values[1].as_integer()
                    .map_err(|e| format!("Value 2: {}", e))?;
                let blue = values[2].as_integer()
                    .map_err(|e| format!("Value 3: {}", e))?;
                let alpha = if has_alpha {
                    let alpha = values[3].as_float()
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

// Re-export of palette's color for convenience so people don't have to add palette to their own
// crate unless they need more complex color functionality.
pub type Color = ::palette::pixel::Srgb;
