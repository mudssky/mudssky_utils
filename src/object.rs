//! Object utilities for manipulation and transformation
//!
//! This module provides utilities for working with objects, including picking,
//! omitting, mapping, merging, and serialization operations.

use serde_json::{Map, Value};
use thiserror::Error;

/// Errors that can occur during object operations
#[derive(Error, Debug, PartialEq)]
pub enum ObjectError {
    #[error("Serialization error: {message}")]
    SerializationError { message: String },
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
}

/// Pick specified keys from a JSON object
///
/// # Arguments
///
/// * `obj` - The source JSON object
/// * `keys` - Vector of keys to pick
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::pick;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "name": "John",
///     "age": 30,
///     "city": "New York"
/// });
/// let keys = vec!["name".to_string(), "age".to_string()];
/// let result = pick(&obj, &keys);
///
/// assert_eq!(result["name"], "John");
/// assert_eq!(result["age"], 30);
/// assert!(!result.as_object().unwrap().contains_key("city"));
/// ```
pub fn pick(obj: &Value, keys: &[String]) -> Value {
    if let Some(map) = obj.as_object() {
        let mut result = Map::new();
        for key in keys {
            if let Some(value) = map.get(key) {
                result.insert(key.clone(), value.clone());
            }
        }
        Value::Object(result)
    } else {
        Value::Object(Map::new())
    }
}

/// Pick keys from a JSON object based on a predicate function
///
/// # Arguments
///
/// * `obj` - The source JSON object
/// * `predicate` - Function that returns true for values to keep
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::pick_by;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "a": 1,
///     "b": null,
///     "c": "hello",
///     "d": false
/// });
///
/// let result = pick_by(&obj, |value| !value.is_null());
/// assert!(result.as_object().unwrap().contains_key("a"));
/// assert!(result.as_object().unwrap().contains_key("c"));
/// assert!(!result.as_object().unwrap().contains_key("b"));
/// ```
pub fn pick_by<F>(obj: &Value, predicate: F) -> Value
where
    F: Fn(&Value) -> bool,
{
    if let Some(map) = obj.as_object() {
        let mut result = Map::new();
        for (key, value) in map {
            if predicate(value) {
                result.insert(key.clone(), value.clone());
            }
        }
        Value::Object(result)
    } else {
        Value::Object(Map::new())
    }
}

/// Omit specified keys from a JSON object
///
/// # Arguments
///
/// * `obj` - The source JSON object
/// * `keys` - Vector of keys to omit
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::omit;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "name": "John",
///     "age": 30,
///     "city": "New York"
/// });
/// let keys = vec!["age".to_string()];
/// let result = omit(&obj, &keys);
///
/// assert_eq!(result["name"], "John");
/// assert_eq!(result["city"], "New York");
/// assert!(!result.as_object().unwrap().contains_key("age"));
/// ```
pub fn omit(obj: &Value, keys: &[String]) -> Value {
    if let Some(map) = obj.as_object() {
        let mut result = Map::new();
        for (key, value) in map {
            if !keys.contains(key) {
                result.insert(key.clone(), value.clone());
            }
        }
        Value::Object(result)
    } else {
        obj.clone()
    }
}

/// Omit keys from a JSON object based on a predicate function
///
/// # Arguments
///
/// * `obj` - The source JSON object
/// * `predicate` - Function that returns true for values to omit
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::omit_by;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "a": 1,
///     "b": null,
///     "c": "hello"
/// });
///
/// let result = omit_by(&obj, |value| value.is_null());
/// assert!(result.as_object().unwrap().contains_key("a"));
/// assert!(result.as_object().unwrap().contains_key("c"));
/// assert!(!result.as_object().unwrap().contains_key("b"));
/// ```
pub fn omit_by<F>(obj: &Value, predicate: F) -> Value
where
    F: Fn(&Value) -> bool,
{
    pick_by(obj, |value| !predicate(value))
}

/// Map object keys using a transformation function
///
/// # Arguments
///
/// * `obj` - The source JSON object
/// * `mapper` - Function to transform keys
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::map_keys;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "firstName": "John",
///     "lastName": "Doe"
/// });
///
/// let result = map_keys(&obj, |key| key.to_uppercase());
/// assert_eq!(result["FIRSTNAME"], "John");
/// assert_eq!(result["LASTNAME"], "Doe");
/// ```
pub fn map_keys<F>(obj: &Value, mapper: F) -> Value
where
    F: Fn(&str) -> String,
{
    if let Some(map) = obj.as_object() {
        let mut result = Map::new();
        for (key, value) in map {
            let new_key = mapper(key);
            result.insert(new_key, value.clone());
        }
        Value::Object(result)
    } else {
        obj.clone()
    }
}

/// Map object values using a transformation function
///
/// # Arguments
///
/// * `obj` - The source JSON object
/// * `mapper` - Function to transform values
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::map_values;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "a": 1,
///     "b": 2,
///     "c": 3
/// });
///
/// let result = map_values(&obj, |value| {
///     if let Some(num) = value.as_i64() {
///         json!(num * 2)
///     } else {
///         value.clone()
///     }
/// });
///
/// assert_eq!(result["a"], 2);
/// assert_eq!(result["b"], 4);
/// assert_eq!(result["c"], 6);
/// ```
pub fn map_values<F>(obj: &Value, mapper: F) -> Value
where
    F: Fn(&Value) -> Value,
{
    if let Some(map) = obj.as_object() {
        let mut result = Map::new();
        for (key, value) in map {
            let new_value = mapper(value);
            result.insert(key.clone(), new_value);
        }
        Value::Object(result)
    } else {
        obj.clone()
    }
}

/// Recursively merge multiple JSON objects
///
/// # Arguments
///
/// * `target` - The target object to merge into
/// * `sources` - Vector of source objects to merge
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::merge;
/// use serde_json::{json, Value};
///
/// let mut target = json!({
///     "a": 1,
///     "b": { "x": 10 }
/// });
///
/// let source1 = json!({
///     "b": { "y": 20 },
///     "c": 3
/// });
///
/// let source2 = json!({
///     "d": 4
/// });
///
/// let result = merge(&mut target, &[source1, source2]);
/// assert_eq!(result["a"], 1);
/// assert_eq!(result["b"]["x"], 10);
/// assert_eq!(result["b"]["y"], 20);
/// assert_eq!(result["c"], 3);
/// assert_eq!(result["d"], 4);
/// ```
pub fn merge<'a>(target: &'a mut Value, sources: &[Value]) -> &'a Value {
    for source in sources {
        merge_recursive(target, source);
    }
    target
}

fn merge_recursive(target: &mut Value, source: &Value) {
    if let (Some(target_map), Some(source_map)) = (target.as_object_mut(), source.as_object()) {
        for (key, value) in source_map {
            if let Some(target_value) = target_map.get_mut(key) {
                if target_value.is_object() && value.is_object() {
                    merge_recursive(target_value, value);
                } else {
                    *target_value = value.clone();
                }
            } else {
                target_map.insert(key.clone(), value.clone());
            }
        }
    }
}

/// Remove non-serializable properties from a JSON value
///
/// # Arguments
///
/// * `obj` - The JSON value to clean
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::remove_non_serializable_props;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "name": "John",
///     "age": 30,
///     "data": null
/// });
///
/// let result = remove_non_serializable_props(&obj);
/// // All properties are already serializable in this example
/// assert_eq!(result["name"], "John");
/// assert_eq!(result["age"], 30);
/// ```
pub fn remove_non_serializable_props(obj: &Value) -> Value {
    match obj {
        Value::Object(map) => {
            let mut result = Map::new();
            for (key, value) in map {
                let cleaned_value = remove_non_serializable_props(value);
                result.insert(key.clone(), cleaned_value);
            }
            Value::Object(result)
        }
        Value::Array(arr) => {
            let cleaned_array: Vec<Value> = arr.iter().map(remove_non_serializable_props).collect();
            Value::Array(cleaned_array)
        }
        _ => obj.clone(),
    }
}

/// Safely stringify a JSON value to string
///
/// # Arguments
///
/// * `obj` - The JSON value to stringify
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::safe_json_stringify;
/// use serde_json::json;
///
/// let obj = json!({
///     "name": "John",
///     "age": 30
/// });
///
/// let result = safe_json_stringify(&obj).unwrap();
/// assert!(result.contains("John"));
/// assert!(result.contains("30"));
/// ```
///
/// # Errors
///
/// Returns `ObjectError::SerializationError` if serialization fails
pub fn safe_json_stringify(obj: &Value) -> Result<String, ObjectError> {
    let cleaned = remove_non_serializable_props(obj);
    serde_json::to_string(&cleaned).map_err(|e| ObjectError::SerializationError {
        message: e.to_string(),
    })
}

/// Invert the keys and values of a JSON object
///
/// # Arguments
///
/// * `obj` - The JSON object to invert
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object::invert;
/// use serde_json::{json, Value};
///
/// let obj = json!({
///     "a": "x",
///     "b": "y",
///     "c": "z"
/// });
///
/// let result = invert(&obj);
/// assert_eq!(result["x"], "a");
/// assert_eq!(result["y"], "b");
/// assert_eq!(result["z"], "c");
/// ```
pub fn invert(obj: &Value) -> Value {
    if let Some(map) = obj.as_object() {
        let mut result = Map::new();
        for (key, value) in map {
            let string_key = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                _ => continue, // Skip arrays and objects
            };
            result.insert(string_key, Value::String(key.clone()));
        }
        Value::Object(result)
    } else {
        Value::Object(Map::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pick() {
        let obj = json!({
            "name": "John",
            "age": 30,
            "city": "New York"
        });
        let keys = vec!["name".to_string(), "age".to_string()];
        let result = pick(&obj, &keys);

        assert_eq!(result["name"], "John");
        assert_eq!(result["age"], 30);
        assert!(!result.as_object().unwrap().contains_key("city"));
    }

    #[test]
    fn test_pick_by() {
        let obj = json!({
            "a": 1,
            "b": null,
            "c": "hello"
        });

        let result = pick_by(&obj, |value| !value.is_null());
        assert!(result.as_object().unwrap().contains_key("a"));
        assert!(result.as_object().unwrap().contains_key("c"));
        assert!(!result.as_object().unwrap().contains_key("b"));
    }

    #[test]
    fn test_omit() {
        let obj = json!({
            "name": "John",
            "age": 30,
            "city": "New York"
        });
        let keys = vec!["age".to_string()];
        let result = omit(&obj, &keys);

        assert_eq!(result["name"], "John");
        assert_eq!(result["city"], "New York");
        assert!(!result.as_object().unwrap().contains_key("age"));
    }

    #[test]
    fn test_map_keys() {
        let obj = json!({
            "firstName": "John",
            "lastName": "Doe"
        });

        let result = map_keys(&obj, |key| key.to_uppercase());
        assert_eq!(result["FIRSTNAME"], "John");
        assert_eq!(result["LASTNAME"], "Doe");
    }

    #[test]
    fn test_map_values() {
        let obj = json!({
            "a": 1,
            "b": 2,
            "c": 3
        });

        let result = map_values(&obj, |value| {
            if let Some(num) = value.as_i64() {
                json!(num * 2)
            } else {
                value.clone()
            }
        });

        assert_eq!(result["a"], 2);
        assert_eq!(result["b"], 4);
        assert_eq!(result["c"], 6);
    }

    #[test]
    fn test_merge() {
        let mut target = json!({
            "a": 1,
            "b": { "x": 10 }
        });

        let source1 = json!({
            "b": { "y": 20 },
            "c": 3
        });

        let source2 = json!({
            "d": 4
        });

        let result = merge(&mut target, &[source1, source2]);
        assert_eq!(result["a"], 1);
        assert_eq!(result["b"]["x"], 10);
        assert_eq!(result["b"]["y"], 20);
        assert_eq!(result["c"], 3);
        assert_eq!(result["d"], 4);
    }

    #[test]
    fn test_invert() {
        let obj = json!({
            "a": "x",
            "b": "y",
            "c": "z"
        });

        let result = invert(&obj);
        assert_eq!(result["x"], "a");
        assert_eq!(result["y"], "b");
        assert_eq!(result["z"], "c");
    }

    #[test]
    fn test_safe_json_stringify() {
        let obj = json!({
            "name": "John",
            "age": 30
        });

        let result = safe_json_stringify(&obj);
        assert!(result.is_ok());
        let json_str = result.unwrap();
        assert!(json_str.contains("John"));
        assert!(json_str.contains("30"));
    }
}
