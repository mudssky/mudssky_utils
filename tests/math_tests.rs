use mudssky_utils::math::*;

#[test]
fn test_random_int_range() {
    for _ in 0..100 {
        let result = random_int(10, 20);
        assert!(result.is_ok());
        let num = result.unwrap();
        assert!(
            num >= 10 && num < 20,
            "Generated number {} is not in range [10, 20)",
            num
        );
    }
}

#[test]
fn test_random_int_negative_range() {
    for _ in 0..100 {
        let result = random_int(-10, 10);
        assert!(result.is_ok());
        let num = result.unwrap();
        assert!(
            num >= -10 && num < 10,
            "Generated number {} is not in range [-10, 10)",
            num
        );
    }
}

#[test]
fn test_random_int_invalid_range() {
    let result = random_int(20, 10);
    assert!(result.is_err());

    let result = random_int(10, 10);
    assert!(result.is_err());
}

#[test]
fn test_random_int_max() {
    for _ in 0..100 {
        let result = random_int_max(50);
        assert!(result.is_ok());
        let num = result.unwrap();
        assert!(
            num >= 0 && num < 50,
            "Generated number {} is not in range [0, 50)",
            num
        );
    }
}

#[test]
fn test_random_int_max_invalid() {
    let result = random_int_max(0);
    assert!(result.is_err());

    let result = random_int_max(-5);
    assert!(result.is_err());
}

#[test]
fn test_get_random_item_from_array() {
    let arr = vec!["apple", "banana", "cherry", "date"];

    for _ in 0..100 {
        let result = get_random_item_from_array(&arr);
        assert!(result.is_ok());
        let item = result.unwrap();
        assert!(arr.contains(&item), "Item {} not found in array", item);
    }
}

#[test]
fn test_get_random_item_from_single_item_array() {
    let arr = vec![42];
    let result = get_random_item_from_array(&arr);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_get_random_item_from_empty_array() {
    let arr: Vec<i32> = vec![];
    let result = get_random_item_from_array(&arr);
    assert!(result.is_err());
}

#[test]
fn test_get_random_item_with_different_types() {
    // Test with strings
    let string_arr = vec!["hello".to_string(), "world".to_string()];
    let result = get_random_item_from_array(&string_arr);
    assert!(result.is_ok());

    // Test with numbers
    let num_arr = vec![1.5, 2.7, 3.14];
    let result = get_random_item_from_array(&num_arr);
    assert!(result.is_ok());
    assert!(num_arr.contains(&result.unwrap()));
}

#[test]
fn test_math_error_display() {
    let error = MathError::InvalidArgument {
        message: "test error".to_string(),
    };
    assert_eq!(error.to_string(), "Invalid argument: test error");
}
