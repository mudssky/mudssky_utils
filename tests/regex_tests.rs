//! Integration tests for regex utilities

use mudssky_utils::regex::*;

#[test]
fn test_username_validation() {
    assert!(is_valid_username("user123"));
    assert!(is_valid_username("test_user"));
    assert!(is_valid_username("user-name"));
    assert!(!is_valid_username("usr")); // too short
    assert!(!is_valid_username("user@name")); // invalid character
}

#[test]
fn test_email_validation() {
    assert!(is_valid_email("test@example.com"));
    assert!(is_valid_email("user.name@domain.co.uk"));
    assert!(!is_valid_email("invalid.email"));
    assert!(!is_valid_email("@domain.com"));
}

#[test]
fn test_mobile_validation() {
    assert!(is_valid_mobile_cn("13812345678"));
    assert!(is_valid_mobile_cn("15987654321"));
    assert!(!is_valid_mobile_cn("12345678901")); // doesn't start with valid prefix
    assert!(!is_valid_mobile_cn("1381234567")); // too short
}

#[test]
fn test_number_validation() {
    assert!(is_positive_number("123"));
    assert!(is_positive_number("123.45"));
    assert!(is_negative_number("-123"));
    assert!(is_negative_number("-123.45"));
    assert!(!is_positive_number("-123"));
    assert!(!is_negative_number("123"));
}

#[test]
fn test_url_validation() {
    assert!(is_valid_url("https://example.com"));
    assert!(is_valid_url("http://test.org"));
    assert!(!is_valid_url("not-a-url"));
    assert!(!is_valid_url("ftp://example.com")); // not http/https
}

#[test]
fn test_ip_validation() {
    assert!(is_valid_ipv4("192.168.1.1"));
    assert!(is_valid_ipv4("255.255.255.255"));
    assert!(!is_valid_ipv4("256.1.1.1")); // invalid range
    assert!(!is_valid_ipv4("192.168.1")); // incomplete

    assert!(is_valid_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    assert!(!is_valid_ipv6("invalid:ipv6"));
}

#[test]
fn test_hex_color_validation() {
    assert!(is_valid_hex_color("#FF0000"));
    assert!(is_valid_hex_color("#f00"));
    assert!(!is_valid_hex_color("FF0000")); // missing #
    assert!(!is_valid_hex_color("#GG0000")); // invalid hex
}

#[test]
fn test_password_strength() {
    let weak = analyze_password_strength("password");
    assert!(weak.min_length);
    assert!(weak.has_lowercase);
    assert!(!weak.has_uppercase);
    assert!(!weak.has_digit);
    assert!(!weak.has_special_char);
    assert_eq!(weak.score, 2);

    let strong = analyze_password_strength("Password123!");
    assert!(strong.min_length);
    assert!(strong.has_lowercase);
    assert!(strong.has_uppercase);
    assert!(strong.has_digit);
    assert!(strong.has_special_char);
    assert_eq!(strong.score, 5);
}

#[test]
fn test_password_strength_level() {
    assert_eq!(calculate_password_strength_level("weak"), 0); // too short
    assert_eq!(calculate_password_strength_level("password"), 1); // min + lowercase
    assert_eq!(calculate_password_strength_level("Password123!"), 4); // all criteria
}

#[test]
fn test_extract_matches() {
    let text = "Contact us at test@example.com or admin@test.org";
    let emails = extract_matches(text, r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap();
    assert_eq!(emails.len(), 2);
    assert!(emails.contains(&"test@example.com".to_string()));
    assert!(emails.contains(&"admin@test.org".to_string()));
}

#[test]
fn test_replace_all_matches() {
    let text = "Phone: 123-456-7890 or 098-765-4321";
    let result = replace_all_matches(text, r"\d{3}-\d{3}-\d{4}", "[REDACTED]").unwrap();
    assert_eq!(result, "Phone: [REDACTED] or [REDACTED]");
}

#[test]
fn test_split_by_pattern() {
    let text = "apple,banana;orange:grape";
    let parts = split_by_pattern(text, r"[,;:]").unwrap();
    assert_eq!(parts, vec!["apple", "banana", "orange", "grape"]);
}

#[test]
fn test_matches_pattern() {
    assert!(matches_pattern("hello123", r"^[a-z]+\d+$").unwrap());
    assert!(!matches_pattern("Hello123", r"^[a-z]+\d+$").unwrap());
}
