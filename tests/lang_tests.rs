//! Integration tests for language utilities

use mudssky_utils::lang::*;
use std::collections::{HashMap, HashSet};

#[test]
fn test_is_empty() {
    assert!(is_empty(&Option::<i32>::None));
    assert!(!is_empty(&Some(42)));
}

#[test]
fn test_is_empty_string() {
    assert!(is_empty_string(""));
    assert!(!is_empty_string("hello"));
}

#[test]
fn test_is_empty_vec() {
    let empty_vec: Vec<i32> = vec![];
    assert!(is_empty_vec(&empty_vec));
    assert!(!is_empty_vec(&[1]));
    assert!(!is_empty_vec(&[1, 2, 3]));
}

#[test]
fn test_is_empty_hashmap() {
    let empty_map: HashMap<String, i32> = HashMap::new();
    assert!(is_empty_hashmap(&empty_map));

    let mut map = HashMap::new();
    map.insert("key".to_string(), 1);
    assert!(!is_empty_hashmap(&map));
}

#[test]
fn test_is_empty_hashset() {
    let empty_set: HashSet<i32> = HashSet::new();
    assert!(is_empty_hashset(&empty_set));

    let mut set = HashSet::new();
    set.insert(1);
    assert!(!is_empty_hashset(&set));
}

#[test]
fn test_is_zero() {
    assert!(is_zero_i32(0));
    assert!(is_zero_f64(0.0));
    assert!(!is_zero_i32(1));
    assert!(!is_zero_f64(0.1));
    assert!(!is_zero_i32(-1));
}

#[test]
fn test_is_some_none() {
    let some_value = Some(42);
    let none_value: Option<i32> = None;

    assert!(is_some(&some_value));
    assert!(!is_some(&none_value));

    assert!(is_none(&none_value));
    assert!(!is_none(&some_value));
}

#[test]
fn test_get_type_name() {
    assert_eq!(get_type_name(&42), "i32");
    assert_eq!(get_type_name(&"hello"), "&str");
    assert_eq!(
        get_type_name(&String::from("hello")),
        "alloc::string::String"
    );
    assert_eq!(get_type_name(&vec![1, 2, 3]), "alloc::vec::Vec<i32>");
}

#[test]
fn test_is_equal() {
    assert!(is_equal(&42, &42));
    assert!(is_equal(&"hello", &"hello"));
    assert!(!is_equal(&42, &43));
    assert!(!is_equal(&"hello", &"world"));

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];
    let vec3 = vec![1, 2, 4];
    assert!(is_equal(&vec1, &vec2));
    assert!(!is_equal(&vec1, &vec3));
}

#[test]
fn test_is_numeric() {
    assert!(is_numeric("123"));
    assert!(!is_numeric("123.45")); // contains dot
    assert!(!is_numeric("-123")); // contains minus
    assert!(!is_numeric("-123.45")); // contains minus and dot
    assert!(is_numeric("0"));
    assert!(!is_numeric("abc"));
    assert!(!is_numeric("12a3"));
    assert!(!is_numeric(""));
}

#[test]
fn test_is_alphabetic() {
    assert!(is_alphabetic("hello"));
    assert!(is_alphabetic("HELLO"));
    assert!(is_alphabetic("HelloWorld"));
    assert!(!is_alphabetic("hello123"));
    assert!(!is_alphabetic("hello world"));
    assert!(!is_alphabetic("hello-world"));
    assert!(!is_alphabetic(""));
}

#[test]
fn test_is_alphanumeric() {
    assert!(is_alphanumeric("hello123"));
    assert!(is_alphanumeric("HELLO123"));
    assert!(is_alphanumeric("abc"));
    assert!(is_alphanumeric("123"));
    assert!(!is_alphanumeric("hello world"));
    assert!(!is_alphanumeric("hello-123"));
    assert!(!is_alphanumeric("hello@123"));
    assert!(!is_alphanumeric(""));
}

#[test]
fn test_is_identifier() {
    assert!(is_identifier("hello"));
    assert!(is_identifier("hello_world"));
    assert!(is_identifier("hello123"));
    assert!(is_identifier("_hello"));
    assert!(is_identifier("HELLO"));
    assert!(!is_identifier("123hello")); // can't start with digit
    assert!(!is_identifier("hello-world")); // hyphen not allowed
    assert!(!is_identifier("hello world")); // space not allowed
    assert!(!is_identifier("")); // empty not allowed
}
