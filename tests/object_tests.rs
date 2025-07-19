use mudssky_utils::object::*;
use serde_json::json;

#[test]
fn test_pick() {
    let obj = json!({
        "name": "Alice",
        "age": 30,
        "city": "New York"
    });

    let keys = vec!["name".to_string(), "age".to_string()];
    let result = pick(&obj, &keys);

    assert_eq!(result["name"], "Alice");
    assert_eq!(result["age"], 30);
    assert!(!result.as_object().unwrap().contains_key("city"));
}

#[test]
fn test_pick_empty_keys() {
    let obj = json!({
        "name": "Alice"
    });

    let keys = vec![];
    let result = pick(&obj, &keys);

    assert!(result.as_object().unwrap().is_empty());
}

#[test]
fn test_pick_by() {
    let obj = json!({
        "name": "Alice",
        "age": 30,
        "score": 85
    });

    let result = pick_by(&obj, |value| value.is_number());

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 2);
    assert!(result_obj.contains_key("age"));
    assert!(result_obj.contains_key("score"));
    assert!(!result_obj.contains_key("name"));
}

#[test]
fn test_omit() {
    let obj = json!({
        "name": "Alice",
        "age": 30,
        "city": "New York"
    });

    let keys = vec!["age".to_string()];
    let result = omit(&obj, &keys);

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 2);
    assert!(result_obj.contains_key("name"));
    assert!(result_obj.contains_key("city"));
    assert!(!result_obj.contains_key("age"));
}

#[test]
fn test_omit_by() {
    let obj = json!({
        "name": "Alice",
        "age": 30,
        "score": 85
    });

    let result = omit_by(&obj, |value| value.is_number());

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 1);
    assert!(result_obj.contains_key("name"));
    assert!(!result_obj.contains_key("age"));
    assert!(!result_obj.contains_key("score"));
}

#[test]
fn test_map_keys() {
    let obj = json!({
        "firstName": "Alice",
        "lastName": "Smith"
    });

    let result = map_keys(&obj, |key| key.to_uppercase());

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 2);
    assert!(result_obj.contains_key("FIRSTNAME"));
    assert!(result_obj.contains_key("LASTNAME"));
    assert_eq!(result["FIRSTNAME"], "Alice");
}

#[test]
fn test_map_values() {
    let obj = json!({
        "name": "alice",
        "city": "new york"
    });

    let result = map_values(&obj, |value| {
        if let Some(s) = value.as_str() {
            json!(s.to_uppercase())
        } else {
            value.clone()
        }
    });

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 2);
    assert_eq!(result["name"], "ALICE");
    assert_eq!(result["city"], "NEW YORK");
}

#[test]
fn test_merge() {
    let mut target = json!({
        "name": "Alice",
        "age": 30
    });

    let sources = vec![json!({
        "age": 31,
        "city": "New York"
    })];

    let result = merge(&mut target, &sources);

    assert_eq!(result["name"], "Alice");
    assert_eq!(result["age"], 31); // sources 的值覆盖了 target
    assert_eq!(result["city"], "New York");
}

#[test]
fn test_remove_non_serializable_props() {
    let obj = json!({
        "name": "Alice",
        "age": 30,
        "func": null
    });

    let result = remove_non_serializable_props(&obj);

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 3); // null 值也是可序列化的
    assert!(result_obj.contains_key("name"));
    assert!(result_obj.contains_key("age"));
    assert!(result_obj.contains_key("func"));
}

#[test]
fn test_safe_json_stringify() {
    let obj = json!({
        "name": "Alice",
        "age": 30
    });

    let result = safe_json_stringify(&obj);

    assert!(result.is_ok());
    let json_str = result.unwrap();
    assert!(json_str.contains("Alice"));
    assert!(json_str.contains("30"));
}

#[test]
fn test_invert() {
    let obj = json!({
        "a": "1",
        "b": "2",
        "c": "1"
    });

    let result = invert(&obj);

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 2);
    assert!(result_obj.contains_key("1"));
    assert!(result_obj.contains_key("2"));
    // 重复值的情况下，后面的键会覆盖前面的
    assert_eq!(result["1"], "c");
    assert_eq!(result["2"], "b");
}

#[test]
fn test_invert_with_non_string_values() {
    let obj = json!({
        "a": 1,
        "b": true,
        "c": null
    });

    let result = invert(&obj);

    let result_obj = result.as_object().unwrap();
    assert_eq!(result_obj.len(), 3);
    assert_eq!(result["1"], "a");
    assert_eq!(result["true"], "b");
    assert_eq!(result["null"], "c");
}

#[test]
fn test_object_error_display() {
    let error = ObjectError::SerializationError {
        message: "test error".to_string(),
    };
    assert_eq!(format!("{error}"), "Serialization error: test error");
}
