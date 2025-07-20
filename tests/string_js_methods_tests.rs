//! Integration tests for JavaScript-like string methods

use mudssky_utils::string::*;

#[test]
fn test_pad_start() {
    assert_eq!(pad_start("5", 3, Some("0")), "005");
    assert_eq!(pad_start("hello", 8, Some(" ")), "   hello");
    assert_eq!(pad_start("world", 3, None), "world"); // no padding needed
    assert_eq!(pad_start("", 3, Some("x")), "xxx");
    assert_eq!(pad_start("test", 6, Some("ab")), "abtest");
}

#[test]
fn test_pad_end() {
    assert_eq!(pad_end("5", 3, Some("0")), "500");
    assert_eq!(pad_end("hello", 8, Some(" ")), "hello   ");
    assert_eq!(pad_end("world", 3, None), "world"); // no padding needed
    assert_eq!(pad_end("", 3, Some("x")), "xxx");
    assert_eq!(pad_end("test", 6, Some("ab")), "testab");
}

#[test]
fn test_repeat() {
    assert_eq!(repeat("abc", 3), "abcabcabc");
    assert_eq!(repeat("x", 0), "");
    assert_eq!(repeat("hello", 2), "hellohello");
    assert_eq!(repeat("", 5), "");
    assert_eq!(repeat("a", 1), "a");
}

#[test]
fn test_starts_with() {
    assert!(starts_with("hello world", "hello"));
    assert!(!starts_with("hello world", "world"));
    assert!(starts_with("test", ""));
    assert!(starts_with("", ""));
    assert!(!starts_with("", "test"));
}

#[test]
fn test_ends_with() {
    assert!(ends_with("hello world", "world"));
    assert!(!ends_with("hello world", "hello"));
    assert!(ends_with("test", ""));
    assert!(ends_with("", ""));
    assert!(!ends_with("", "test"));
}

#[test]
fn test_includes() {
    assert!(includes("hello world", "lo wo"));
    assert!(!includes("hello world", "xyz"));
    assert!(includes("test", ""));
    assert!(includes("", ""));
    assert!(!includes("", "test"));
    assert!(includes("hello", "hello"));
}

#[test]
fn test_char_at() {
    assert_eq!(char_at("hello", 1), Some('e'));
    assert_eq!(char_at("hello", 10), None);
    assert_eq!(char_at("🦀", 0), Some('🦀'));
    assert_eq!(char_at("", 0), None);
    assert_eq!(char_at("test", 3), Some('t'));
}

#[test]
fn test_substring() {
    assert_eq!(substring("hello world", 0, Some(5)), "hello");
    assert_eq!(substring("hello world", 6, None), "world");
    assert_eq!(substring("hello", 1, Some(4)), "ell");
    assert_eq!(substring("test", 0, Some(0)), "");
    assert_eq!(substring("test", 2, Some(2)), "");
    assert_eq!(substring("test", 10, Some(20)), "");
    assert_eq!(substring("hello", 0, None), "hello");
}

#[test]
fn test_split() {
    assert_eq!(split("a,b,c", ","), vec!["a", "b", "c"]);
    assert_eq!(split("hello world", " "), vec!["hello", "world"]);
    assert_eq!(split("test", ","), vec!["test"]);
    assert_eq!(split("", ","), vec![""]);
    assert_eq!(split("a,,b", ","), vec!["a", "", "b"]);
    assert_eq!(split("abc", ""), vec!["a", "b", "c"]);
}

#[test]
fn test_replace_all() {
    assert_eq!(
        replace_all("hello world hello", "hello", "hi"),
        "hi world hi"
    );
    assert_eq!(replace_all("test", "xyz", "abc"), "test");
    assert_eq!(replace_all("aaa", "a", "b"), "bbb");
    assert_eq!(replace_all("", "a", "b"), "");
    assert_eq!(replace_all("test", "", "x"), "test");
}

#[test]
fn test_string_methods_integration() {
    // Test chaining-like operations
    let text = "  hello world  ";
    let processed = pad_start(&trim(text, None), 15, Some("*"));
    assert_eq!(processed, "****hello world");

    // Test with unicode
    let unicode_text = "🦀 Rust";
    assert_eq!(char_at(unicode_text, 0), Some('🦀'));
    assert_eq!(substring(unicode_text, 2, None), "Rust");

    // Test edge cases
    let empty = "";
    assert_eq!(pad_start(empty, 3, Some("x")), "xxx");
    assert_eq!(repeat(empty, 5), "");
    assert!(starts_with(empty, ""));
    assert!(ends_with(empty, ""));
}
