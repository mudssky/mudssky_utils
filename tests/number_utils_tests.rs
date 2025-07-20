//! Integration tests for number utility functions

use mudssky_utils::number_utils::*;

#[test]
fn test_is_finite() {
    assert!(is_finite(42.0));
    assert!(is_finite(-42.0));
    assert!(is_finite(0.0));
    assert!(is_finite(std::f64::consts::PI));
    assert!(!is_finite(f64::INFINITY));
    assert!(!is_finite(f64::NEG_INFINITY));
    assert!(!is_finite(f64::NAN));
}

#[test]
fn test_is_nan() {
    assert!(!is_nan(42.0));
    assert!(!is_nan(0.0));
    assert!(!is_nan(f64::INFINITY));
    assert!(!is_nan(f64::NEG_INFINITY));
    assert!(is_nan(f64::NAN));
    assert!(is_nan(f64::NAN));
    assert!(is_nan(f64::INFINITY - f64::INFINITY));
}

#[test]
fn test_is_integer() {
    assert!(is_integer(42.0));
    assert!(is_integer(-42.0));
    assert!(is_integer(0.0));
    assert!(is_integer(-0.0));
    assert!(!is_integer(42.5));
    assert!(!is_integer(std::f64::consts::PI));
    assert!(!is_integer(f64::NAN));
    assert!(!is_integer(f64::INFINITY));
    assert!(!is_integer(f64::NEG_INFINITY));
}

#[test]
fn test_is_safe_integer() {
    assert!(is_safe_integer(42.0));
    assert!(is_safe_integer(-42.0));
    assert!(is_safe_integer(0.0));
    assert!(is_safe_integer(9007199254740991.0)); // MAX_SAFE_INTEGER
    assert!(is_safe_integer(-9007199254740991.0)); // MIN_SAFE_INTEGER
    assert!(!is_safe_integer(9007199254740992.0)); // MAX_SAFE_INTEGER + 1
    assert!(!is_safe_integer(-9007199254740992.0)); // MIN_SAFE_INTEGER - 1
    assert!(!is_safe_integer(42.5));
    assert!(!is_safe_integer(f64::NAN));
    assert!(!is_safe_integer(f64::INFINITY));
}

#[test]
fn test_parse_float() {
    assert_eq!(parse_float("42.5"), Ok(42.5));
    assert_eq!(parse_float("42"), Ok(42.0));
    assert_eq!(parse_float("-42.5"), Ok(-42.5));
    assert_eq!(parse_float("+42.5"), Ok(42.5));
    assert_eq!(parse_float("123.456"), Ok(123.456));
    assert_eq!(parse_float("42.5abc"), Ok(42.5));
    assert_eq!(parse_float("  42.5  "), Ok(42.5));
    assert_eq!(parse_float("1e10"), Ok(1e10));
    assert_eq!(parse_float("1.5e-10"), Ok(1.5e-10));
    assert_eq!(parse_float("1E+5"), Ok(1e5));

    assert!(parse_float("abc").is_err());
    assert!(parse_float("").is_err());
    assert!(parse_float("   ").is_err());
    assert!(parse_float("+").is_err());
    assert!(parse_float("-").is_err());
}

#[test]
fn test_parse_int() {
    // Base 10
    assert_eq!(parse_int("42", 10), Ok(42));
    assert_eq!(parse_int("-42", 10), Ok(-42));
    assert_eq!(parse_int("+42", 10), Ok(42));
    assert_eq!(parse_int("42abc", 10), Ok(42));
    assert_eq!(parse_int("  42  ", 10), Ok(42));

    // Base 2
    assert_eq!(parse_int("101", 2), Ok(5));
    assert_eq!(parse_int("1010", 2), Ok(10));
    assert_eq!(parse_int("-101", 2), Ok(-5));

    // Base 16
    assert_eq!(parse_int("ff", 16), Ok(255));
    assert_eq!(parse_int("FF", 16), Ok(255));
    assert_eq!(parse_int("10", 16), Ok(16));
    assert_eq!(parse_int("-ff", 16), Ok(-255));

    // Base 8
    assert_eq!(parse_int("77", 8), Ok(63));
    assert_eq!(parse_int("10", 8), Ok(8));

    // Base 36
    assert_eq!(parse_int("z", 36), Ok(35));
    assert_eq!(parse_int("10", 36), Ok(36));

    // Error cases
    assert!(parse_int("abc", 10).is_err());
    assert!(parse_int("", 10).is_err());
    assert!(parse_int("   ", 10).is_err());
    assert!(parse_int("42", 1).is_err()); // invalid radix
    assert!(parse_int("42", 37).is_err()); // invalid radix
    assert!(parse_int("+", 10).is_err());
    assert!(parse_int("-", 10).is_err());
    assert!(parse_int("2", 2).is_err()); // digit >= radix
}

#[test]
fn test_to_fixed() {
    assert_eq!(to_fixed(42.12345, 2), "42.12");
    assert_eq!(to_fixed(42.0, 2), "42.00");
    assert_eq!(to_fixed(42.999, 2), "43.00");
    assert_eq!(to_fixed(42.12345, 0), "42");
    assert_eq!(to_fixed(42.12345, 4), "42.1234");
    assert_eq!(to_fixed(-42.12345, 2), "-42.12");
    assert_eq!(to_fixed(0.0, 3), "0.000");
}

#[test]
fn test_to_exponential() {
    assert_eq!(to_exponential(42.0, Some(2)), "4.20e1");
    assert_eq!(to_exponential(0.00042, Some(2)), "4.20e-4");
    assert_eq!(to_exponential(1234.0, Some(3)), "1.234e3");
    assert_eq!(to_exponential(-42.0, Some(1)), "-4.2e1");

    // Test without specified digits
    let result = to_exponential(42.0, None);
    assert!(result.contains("e"));
    assert!(result.starts_with("4"));
}

#[test]
fn test_to_precision() {
    assert_eq!(to_precision(42.12345, Some(4)), "42.12");
    assert_eq!(to_precision(0.00042, Some(2)), "4.2e-4");
    assert_eq!(to_precision(1234.0, Some(3)), "1.23e3");
    assert_eq!(to_precision(42.0, Some(2)), "42");
    assert_eq!(to_precision(0.0, Some(3)), "000");

    // Test without precision
    let result = to_precision(42.12345, None);
    assert_eq!(result, "42.12345");
}

#[test]
fn test_constants() {
    assert_eq!(max_safe_integer(), 9007199254740991.0);
    assert_eq!(min_safe_integer(), -9007199254740991.0);
    assert_eq!(positive_infinity(), f64::INFINITY);
    assert_eq!(negative_infinity(), f64::NEG_INFINITY);

    // Test that constants are consistent with is_safe_integer
    assert!(is_safe_integer(max_safe_integer()));
    assert!(is_safe_integer(min_safe_integer()));
    assert!(!is_safe_integer(max_safe_integer() + 1.0));
    assert!(!is_safe_integer(min_safe_integer() - 1.0));
}

#[test]
fn test_clamp() {
    assert_eq!(clamp(5.0, 1.0, 10.0), 5.0);
    assert_eq!(clamp(0.0, 1.0, 10.0), 1.0);
    assert_eq!(clamp(15.0, 1.0, 10.0), 10.0);
    assert_eq!(clamp(1.0, 1.0, 10.0), 1.0); // at min
    assert_eq!(clamp(10.0, 1.0, 10.0), 10.0); // at max
    assert_eq!(clamp(-5.0, -10.0, -1.0), -5.0);
    assert_eq!(clamp(-15.0, -10.0, -1.0), -10.0);
    assert_eq!(clamp(0.0, -10.0, -1.0), -1.0);
}

#[test]
fn test_lerp() {
    assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
    assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
    assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    assert_eq!(lerp(10.0, 20.0, 0.5), 15.0);
    assert_eq!(lerp(-10.0, 10.0, 0.5), 0.0);
    assert_eq!(lerp(0.0, 10.0, 0.25), 2.5);
    assert_eq!(lerp(0.0, 10.0, 0.75), 7.5);

    // Test extrapolation
    assert_eq!(lerp(0.0, 10.0, 1.5), 15.0);
    assert_eq!(lerp(0.0, 10.0, -0.5), -5.0);
}

#[test]
fn test_map_range() {
    assert_eq!(map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
    assert_eq!(map_range(0.0, 0.0, 10.0, 0.0, 100.0), 0.0);
    assert_eq!(map_range(10.0, 0.0, 10.0, 0.0, 100.0), 100.0);
    assert_eq!(map_range(2.5, 0.0, 10.0, 0.0, 100.0), 25.0);
    assert_eq!(map_range(7.5, 0.0, 10.0, 0.0, 100.0), 75.0);

    // Test different ranges
    assert_eq!(map_range(50.0, 0.0, 100.0, -1.0, 1.0), 0.0);
    assert_eq!(map_range(0.0, 0.0, 100.0, -1.0, 1.0), -1.0);
    assert_eq!(map_range(100.0, 0.0, 100.0, -1.0, 1.0), 1.0);

    // Test negative ranges
    assert_eq!(map_range(-5.0, -10.0, 0.0, 0.0, 100.0), 50.0);
    assert_eq!(map_range(-10.0, -10.0, 0.0, 0.0, 100.0), 0.0);
    assert_eq!(map_range(0.0, -10.0, 0.0, 0.0, 100.0), 100.0);
}

#[test]
fn test_number_utils_integration() {
    // Test a complex scenario: temperature conversion and validation
    let celsius_temps = vec!["0", "25.5", "100", "-40", "invalid", "37.5abc"];

    let mut valid_temps = Vec::new();
    for temp_str in celsius_temps {
        if let Ok(celsius) = parse_float(temp_str) {
            if is_finite(celsius) {
                let fahrenheit = map_range(celsius, 0.0, 100.0, 32.0, 212.0);
                let clamped = clamp(fahrenheit, -40.0, 212.0);
                valid_temps.push((celsius, clamped));
            }
        }
    }

    assert_eq!(valid_temps.len(), 5); // "invalid" should be filtered out
    assert_eq!(valid_temps[0], (0.0, 32.0)); // 0°C = 32°F
    assert_eq!(valid_temps[2], (100.0, 212.0)); // 100°C = 212°F

    // Test precision formatting
    let pi = std::f64::consts::PI;
    assert_eq!(to_fixed(pi, 2), "3.14");
    assert_eq!(to_precision(pi, Some(4)), "3.142");

    // Test safe integer validation
    let large_number = 9007199254740992.0; // MAX_SAFE_INTEGER + 1
    assert!(!is_safe_integer(large_number));
    assert!(is_finite(large_number));
    assert!(is_integer(large_number));

    // Test interpolation for animation
    let start_pos = 0.0;
    let end_pos = 100.0;
    let animation_progress = 0.3;
    let current_pos = lerp(start_pos, end_pos, animation_progress);
    assert_eq!(current_pos, 30.0);

    // Test parsing different number formats
    assert_eq!(parse_int("ff", 16), Ok(255));
    assert_eq!(parse_int("377", 8), Ok(255));
    assert_eq!(parse_int("11111111", 2), Ok(255));

    // All should represent the same number (255)
    let hex_255 = parse_int("ff", 16).unwrap() as f64;
    let oct_255 = parse_int("377", 8).unwrap() as f64;
    let bin_255 = parse_int("11111111", 2).unwrap() as f64;

    assert_eq!(hex_255, 255.0);
    assert_eq!(oct_255, 255.0);
    assert_eq!(bin_255, 255.0);
    assert!(is_integer(hex_255));
    assert!(is_safe_integer(hex_255));
}

#[test]
fn test_edge_cases() {
    // Test with very small numbers
    let tiny = 1e-100;
    assert!(is_finite(tiny));
    assert!(!is_integer(tiny));
    assert!(!is_safe_integer(tiny));

    // Test with very large numbers
    let huge = 1e100;
    assert!(is_finite(huge));
    assert!(is_integer(huge));
    assert!(!is_safe_integer(huge));

    // Test clamp with equal min/max
    assert_eq!(clamp(5.0, 3.0, 3.0), 3.0);
    assert_eq!(clamp(1.0, 3.0, 3.0), 3.0);

    // Test lerp with same start and end
    assert_eq!(lerp(5.0, 5.0, 0.5), 5.0);
    assert_eq!(lerp(5.0, 5.0, 0.0), 5.0);
    assert_eq!(lerp(5.0, 5.0, 1.0), 5.0);

    // Test map_range with same input range
    let result = map_range(5.0, 5.0, 5.0, 0.0, 100.0);
    assert!(result.is_nan() || result.is_infinite()); // Division by zero case
}
