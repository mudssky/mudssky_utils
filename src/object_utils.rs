//! Object utility functions similar to JavaScript object methods
//!
//! This module provides utility functions for working with objects and data structures
//! that are commonly available in JavaScript but not natively in Rust.

use std::collections::HashMap;
use std::hash::Hash;

/// Error type for object operations
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectUtilsError {
    /// Key not found in object
    KeyNotFound(String),
    /// Invalid operation
    InvalidOperation(String),
}

impl std::fmt::Display for ObjectUtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectUtilsError::KeyNotFound(key) => write!(f, "Key not found: {key}"),
            ObjectUtilsError::InvalidOperation(msg) => write!(f, "Invalid operation: {msg}"),
        }
    }
}

impl std::error::Error for ObjectUtilsError {}

/// Get all keys from a HashMap
/// Similar to JavaScript's Object.keys()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::keys;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// let mut result = keys(&map);
/// result.sort();
/// assert_eq!(result, vec![&"age", &"name"]);
/// ```
pub fn keys<K, V>(map: &HashMap<K, V>) -> Vec<&K>
where
    K: Hash + Eq,
{
    map.keys().collect()
}

/// Get all values from a HashMap
/// Similar to JavaScript's Object.values()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::values;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// let mut result = values(&map);
/// result.sort();
/// assert_eq!(result, vec![&"30", &"John"]);
/// ```
pub fn values<K, V>(map: &HashMap<K, V>) -> Vec<&V>
where
    K: Hash + Eq,
{
    map.values().collect()
}

/// Get all key-value pairs from a HashMap
/// Similar to JavaScript's Object.entries()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::entries;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// let mut result = entries(&map);
/// result.sort_by_key(|(k, _)| *k);
/// assert_eq!(result, vec![(&"age", &"30"), (&"name", &"John")]);
/// ```
pub fn entries<K, V>(map: &HashMap<K, V>) -> Vec<(&K, &V)>
where
    K: Hash + Eq,
{
    map.iter().collect()
}

/// Check if a HashMap has a specific key
/// Similar to JavaScript's Object.hasOwnProperty()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::has_key;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// assert!(has_key(&map, &"name"));
/// assert!(!has_key(&map, &"age"));
/// ```
pub fn has_key<K, V>(map: &HashMap<K, V>, key: &K) -> bool
where
    K: Hash + Eq,
{
    map.contains_key(key)
}

/// Create a HashMap from key-value pairs
/// Similar to JavaScript's Object.fromEntries()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::from_entries;
///
/// let entries = vec![("name", "John"), ("age", "30")];
/// let map = from_entries(entries);
/// assert_eq!(map.get("name"), Some(&"John"));
/// assert_eq!(map.get("age"), Some(&"30"));
/// ```
pub fn from_entries<K, V>(entries: Vec<(K, V)>) -> HashMap<K, V>
where
    K: Hash + Eq,
{
    entries.into_iter().collect()
}

/// Assign properties from source maps to target map
/// Similar to JavaScript's Object.assign()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::assign;
/// use std::collections::HashMap;
///
/// let mut target = HashMap::new();
/// target.insert("a", 1);
/// target.insert("b", 2);
///
/// let mut source1 = HashMap::new();
/// source1.insert("b", 3);
/// source1.insert("c", 4);
///
/// let mut source2 = HashMap::new();
/// source2.insert("c", 5);
/// source2.insert("d", 6);
///
/// assign(&mut target, vec![&source1, &source2]);
/// assert_eq!(target.get("a"), Some(&1));
/// assert_eq!(target.get("b"), Some(&3));
/// assert_eq!(target.get("c"), Some(&5));
/// assert_eq!(target.get("d"), Some(&6));
/// ```
pub fn assign<K, V>(target: &mut HashMap<K, V>, sources: Vec<&HashMap<K, V>>)
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    for source in sources {
        for (key, value) in source {
            target.insert(key.clone(), value.clone());
        }
    }
}

/// Pick specific keys from a HashMap
/// Similar to lodash's pick() function
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::pick;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// map.insert("city", "NYC");
///
/// let picked = pick(&map, &["name", "age"]);
/// assert_eq!(picked.len(), 2);
/// assert_eq!(picked.get("name"), Some(&"John"));
/// assert_eq!(picked.get("age"), Some(&"30"));
/// assert_eq!(picked.get("city"), None);
/// ```
pub fn pick<K, V>(map: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    let mut result = HashMap::new();
    for key in keys {
        if let Some(value) = map.get(key) {
            result.insert(key.clone(), value.clone());
        }
    }
    result
}

/// Omit specific keys from a HashMap
/// Similar to lodash's omit() function
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::omit;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// map.insert("city", "NYC");
///
/// let omitted = omit(&map, &["age"]);
/// assert_eq!(omitted.len(), 2);
/// assert_eq!(omitted.get("name"), Some(&"John"));
/// assert_eq!(omitted.get("city"), Some(&"NYC"));
/// assert_eq!(omitted.get("age"), None);
/// ```
pub fn omit<K, V>(map: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    let mut result = HashMap::new();
    for (key, value) in map {
        if !keys.contains(key) {
            result.insert(key.clone(), value.clone());
        }
    }
    result
}

/// Deep clone a nested HashMap structure
/// Similar to lodash's cloneDeep() for objects
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::deep_clone;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
///
/// let cloned = deep_clone(&map);
/// assert_eq!(cloned, map);
/// ```
pub fn deep_clone<K, V>(map: &HashMap<K, V>) -> HashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    map.clone()
}

/// Check if a HashMap is empty
/// Similar to lodash's isEmpty() for objects
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::is_empty;
/// use std::collections::HashMap;
///
/// let empty_map: HashMap<String, String> = HashMap::new();
/// assert!(is_empty(&empty_map));
///
/// let mut non_empty_map = HashMap::new();
/// non_empty_map.insert("key", "value");
/// assert!(!is_empty(&non_empty_map));
/// ```
pub fn is_empty<K, V>(map: &HashMap<K, V>) -> bool
where
    K: Hash + Eq,
{
    map.is_empty()
}

/// Get the size/length of a HashMap
/// Similar to JavaScript's Object.keys().length
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::size;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("name", "John");
/// map.insert("age", "30");
/// assert_eq!(size(&map), 2);
/// ```
pub fn size<K, V>(map: &HashMap<K, V>) -> usize
where
    K: Hash + Eq,
{
    map.len()
}

/// Merge multiple HashMaps into a new one
/// Similar to JavaScript's spread operator {...obj1, ...obj2}
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::object_utils::merge;
/// use std::collections::HashMap;
///
/// let mut map1 = HashMap::new();
/// map1.insert("a", 1);
/// map1.insert("b", 2);
///
/// let mut map2 = HashMap::new();
/// map2.insert("b", 3);
/// map2.insert("c", 4);
///
/// let merged = merge(&[&map1, &map2]);
/// assert_eq!(merged.get("a"), Some(&1));
/// assert_eq!(merged.get("b"), Some(&3)); // map2 overwrites map1
/// assert_eq!(merged.get("c"), Some(&4));
/// ```
pub fn merge<K, V>(maps: &[&HashMap<K, V>]) -> HashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    let mut result = HashMap::new();
    for map in maps {
        for (key, value) in *map {
            result.insert(key.clone(), value.clone());
        }
    }
    result
}
