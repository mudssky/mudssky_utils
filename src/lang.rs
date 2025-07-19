//! Language utilities for type checking and value inspection
//!
//! This module provides utilities for checking types and inspecting values,
//! similar to JavaScript's type checking functions.

use std::collections::{HashMap, HashSet};

/// Check if a value is empty
///
/// Returns true for:
/// - None values
/// - Empty strings
/// - Empty vectors
/// - Empty HashMaps
/// - Empty HashSets
/// - Zero numbers
pub fn is_empty<T>(value: &Option<T>) -> bool {
    value.is_none()
}

/// Check if a string is empty
pub fn is_empty_string(value: &str) -> bool {
    value.is_empty()
}

/// Check if a vector is empty
pub fn is_empty_vec<T>(value: &[T]) -> bool {
    value.is_empty()
}

/// Check if a HashMap is empty
pub fn is_empty_hashmap<K, V>(value: &HashMap<K, V>) -> bool {
    value.is_empty()
}

/// Check if a HashSet is empty
pub fn is_empty_hashset<T>(value: &HashSet<T>) -> bool {
    value.is_empty()
}

/// Check if a number is zero
pub fn is_zero_i32(value: i32) -> bool {
    value == 0
}

/// Check if a number is zero
pub fn is_zero_f64(value: f64) -> bool {
    value == 0.0
}

/// Check if a value is Some
pub fn is_some<T>(value: &Option<T>) -> bool {
    value.is_some()
}

/// Check if a value is None
pub fn is_none<T>(value: &Option<T>) -> bool {
    value.is_none()
}

/// Get the type name of a value
pub fn get_type_name<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Check if two values are equal
pub fn is_equal<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

/// Check if a string contains only digits
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

/// Check if a string contains only alphabetic characters
pub fn is_alphabetic(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
}

/// Check if a string contains only alphanumeric characters
pub fn is_alphanumeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric())
}

/// Check if a string is a valid identifier (starts with letter or underscore, followed by letters, digits, or underscores)
pub fn is_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}
