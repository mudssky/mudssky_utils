//! Integration tests for object utility functions

use mudssky_utils::object_utils::*;
use std::collections::HashMap;

#[test]
fn test_keys() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "NYC");

    let mut result = keys(&map);
    result.sort();
    assert_eq!(result, vec![&"age", &"city", &"name"]);

    let empty_map: HashMap<String, String> = HashMap::new();
    let empty_keys = keys(&empty_map);
    assert_eq!(empty_keys.len(), 0);
}

#[test]
fn test_values() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "NYC");

    let mut result = values(&map);
    result.sort();
    assert_eq!(result, vec![&"30", &"John", &"NYC"]);

    let empty_map: HashMap<String, String> = HashMap::new();
    let empty_values = values(&empty_map);
    assert_eq!(empty_values.len(), 0);
}

#[test]
fn test_entries() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    let mut result = entries(&map);
    result.sort_by_key(|(k, _)| *k);
    assert_eq!(result, vec![(&"age", &"30"), (&"name", &"John")]);

    let empty_map: HashMap<String, String> = HashMap::new();
    let empty_entries = entries(&empty_map);
    assert_eq!(empty_entries.len(), 0);
}

#[test]
fn test_has_key() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");

    assert!(has_key(&map, &"name"));
    assert!(has_key(&map, &"age"));
    assert!(!has_key(&map, &"city"));

    let empty_map: HashMap<String, String> = HashMap::new();
    assert!(!has_key(&empty_map, &"anything".to_string()));
}

#[test]
fn test_from_entries() {
    let entries = vec![("name", "John"), ("age", "30"), ("city", "NYC")];
    let map = from_entries(entries);

    assert_eq!(map.get("name"), Some(&"John"));
    assert_eq!(map.get("age"), Some(&"30"));
    assert_eq!(map.get("city"), Some(&"NYC"));
    assert_eq!(map.len(), 3);

    let empty_entries: Vec<(String, String)> = vec![];
    let empty_map = from_entries(empty_entries);
    assert_eq!(empty_map.len(), 0);
}

#[test]
fn test_assign() {
    let mut target = HashMap::new();
    target.insert("a", 1);
    target.insert("b", 2);

    let mut source1 = HashMap::new();
    source1.insert("b", 3);
    source1.insert("c", 4);

    let mut source2 = HashMap::new();
    source2.insert("c", 5);
    source2.insert("d", 6);

    assign(&mut target, vec![&source1, &source2]);

    assert_eq!(target.get("a"), Some(&1));
    assert_eq!(target.get("b"), Some(&3)); // overwritten by source1
    assert_eq!(target.get("c"), Some(&5)); // overwritten by source2
    assert_eq!(target.get("d"), Some(&6));
    assert_eq!(target.len(), 4);
}

#[test]
fn test_pick() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "NYC");
    map.insert("country", "USA");

    let picked = pick(&map, &["name", "age", "nonexistent"]);

    assert_eq!(picked.len(), 2);
    assert_eq!(picked.get("name"), Some(&"John"));
    assert_eq!(picked.get("age"), Some(&"30"));
    assert_eq!(picked.get("city"), None);
    assert_eq!(picked.get("country"), None);
    assert_eq!(picked.get("nonexistent"), None);
}

#[test]
fn test_omit() {
    let mut map = HashMap::new();
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "NYC");
    map.insert("country", "USA");

    let omitted = omit(&map, &["age", "country"]);

    assert_eq!(omitted.len(), 2);
    assert_eq!(omitted.get("name"), Some(&"John"));
    assert_eq!(omitted.get("city"), Some(&"NYC"));
    assert_eq!(omitted.get("age"), None);
    assert_eq!(omitted.get("country"), None);
}

#[test]
fn test_deep_clone() {
    let mut original = HashMap::new();
    original.insert("name", "John");
    original.insert("age", "30");

    let cloned = deep_clone(&original);

    assert_eq!(cloned, original);
    assert_eq!(cloned.get("name"), Some(&"John"));
    assert_eq!(cloned.get("age"), Some(&"30"));

    // Verify they are separate instances (different memory addresses)
    assert_ne!(&cloned as *const _, &original as *const _);
}

#[test]
fn test_is_empty() {
    let empty_map: HashMap<String, String> = HashMap::new();
    assert!(is_empty(&empty_map));

    let mut non_empty_map = HashMap::new();
    non_empty_map.insert("key", "value");
    assert!(!is_empty(&non_empty_map));
}

#[test]
fn test_size() {
    let mut map = HashMap::new();
    assert_eq!(size(&map), 0);

    map.insert("name", "John");
    assert_eq!(size(&map), 1);

    map.insert("age", "30");
    assert_eq!(size(&map), 2);

    map.remove("name");
    assert_eq!(size(&map), 1);
}

#[test]
fn test_merge() {
    let mut map1 = HashMap::new();
    map1.insert("a", 1);
    map1.insert("b", 2);

    let mut map2 = HashMap::new();
    map2.insert("b", 3);
    map2.insert("c", 4);

    let mut map3 = HashMap::new();
    map3.insert("c", 5);
    map3.insert("d", 6);

    let merged = merge(&[&map1, &map2, &map3]);

    assert_eq!(merged.get("a"), Some(&1));
    assert_eq!(merged.get("b"), Some(&3)); // map2 overwrites map1
    assert_eq!(merged.get("c"), Some(&5)); // map3 overwrites map2
    assert_eq!(merged.get("d"), Some(&6));
    assert_eq!(merged.len(), 4);

    // Test with empty array
    let empty_merge: HashMap<&str, i32> = merge(&[]);
    assert_eq!(empty_merge.len(), 0);

    // Test with single map
    let single_merge = merge(&[&map1]);
    assert_eq!(single_merge, map1);
}

#[test]
fn test_object_utils_integration() {
    // Create a complex scenario
    let mut user_data = HashMap::new();
    user_data.insert("id", "123");
    user_data.insert("name", "John Doe");
    user_data.insert("email", "john@example.com");
    user_data.insert("password", "secret123");
    user_data.insert("role", "admin");
    user_data.insert("created_at", "2023-01-01");

    // Pick only safe fields for API response
    let safe_fields = pick(&user_data, &["id", "name", "email", "role", "created_at"]);
    assert!(!has_key(&safe_fields, &"password"));
    assert_eq!(size(&safe_fields), 5);

    // Create public profile by omitting sensitive data
    let public_profile = omit(&user_data, &["password", "email"]);
    assert_eq!(size(&public_profile), 4);
    assert!(has_key(&public_profile, &"name"));
    assert!(!has_key(&public_profile, &"password"));

    // Merge with additional data
    let mut additional_data = HashMap::new();
    additional_data.insert("last_login", "2023-12-01");
    additional_data.insert("status", "active");

    let complete_profile = merge(&[&public_profile, &additional_data]);
    assert_eq!(size(&complete_profile), 6);
    assert!(has_key(&complete_profile, &"last_login"));
    assert!(has_key(&complete_profile, &"status"));

    // Test entries and reconstruction
    let profile_entries = entries(&complete_profile);
    let reconstructed = from_entries(profile_entries.into_iter().map(|(k, v)| (*k, *v)).collect());
    assert_eq!(reconstructed, complete_profile);
}

#[test]
fn test_with_different_types() {
    // Test with integer keys and values
    let mut int_map = HashMap::new();
    int_map.insert(1, 100);
    int_map.insert(2, 200);
    int_map.insert(3, 300);

    assert!(has_key(&int_map, &2));
    assert_eq!(size(&int_map), 3);

    let picked_ints = pick(&int_map, &[1, 3]);
    assert_eq!(size(&picked_ints), 2);
    assert_eq!(picked_ints.get(&1), Some(&100));
    assert_eq!(picked_ints.get(&3), Some(&300));
    assert_eq!(picked_ints.get(&2), None);

    // Test with mixed operations
    let mut another_int_map = HashMap::new();
    another_int_map.insert(3, 333);
    another_int_map.insert(4, 400);

    let merged_ints = merge(&[&int_map, &another_int_map]);
    assert_eq!(merged_ints.get(&3), Some(&333)); // overwritten
    assert_eq!(merged_ints.get(&4), Some(&400)); // new
    assert_eq!(size(&merged_ints), 4);
}
