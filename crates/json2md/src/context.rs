use minijinja::{Error, ErrorKind, Value};
use serde_json::{Number, Value as JsonValue};

// Direct conversion keeps numeric values independent of Serde serialization features.
pub(crate) fn from_json(input: &JsonValue) -> Result<Value, Error> {
    match input {
        JsonValue::Null => Ok(Value::from(())),
        JsonValue::Bool(value) => Ok(Value::from(*value)),
        JsonValue::String(value) => Ok(Value::from(value.as_str())),
        JsonValue::Number(value) => number(value),
        JsonValue::Array(values) => values.iter().map(from_json).collect(),
        JsonValue::Object(values) => values
            .iter()
            .map(|(key, value)| Ok((key.as_str(), from_json(value)?)))
            .collect(),
    }
}

fn number(number: &Number) -> Result<Value, Error> {
    if let Some(value) = number.as_i64() {
        return Ok(Value::from(value));
    }
    if let Some(value) = number.as_u64() {
        return Ok(Value::from(value));
    }
    if let Some(value) = number.as_i128() {
        return Ok(Value::from(value));
    }
    if let Some(value) = number.as_u128() {
        return Ok(Value::from(value));
    }
    if number.is_f64()
        && let Some(value) = number.as_f64()
    {
        return Ok(Value::from(value));
    }

    Err(Error::new(
        ErrorKind::InvalidOperation,
        "JSON number exceeds the renderer's integer or finite floating-point range",
    ))
}
