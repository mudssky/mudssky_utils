//! Regular expression utilities and common patterns
//!
//! This module provides commonly used regex patterns and validation functions.

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

/// Common regex patterns
#[derive(Debug, Clone)]
pub struct RegexPatterns {
    pub username: Regex,
    pub email: Regex,
    pub email_cn: Regex,
    pub mobile_cn: Regex,
    pub positive_number: Regex,
    pub negative_number: Regex,
    pub url: Regex,
    pub ipv4: Regex,
    pub ipv6: Regex,
    pub hex_color: Regex,
    pub credit_card: Regex,
    pub phone_us: Regex,
}

/// Global instance of regex patterns
static REGEX_PATTERNS: Lazy<RegexPatterns> = Lazy::new(|| {
    RegexPatterns {
        username: Regex::new(r"^[a-zA-Z0-9_-]{4,16}$").unwrap(),
        email: Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap(),
        email_cn: Regex::new(r"^[A-Za-z0-9\u4e00-\u9fa5]+@[a-zA-Z0-9_-]+(\.[a-zA-Z0-9_-]+)+$").unwrap(),
        mobile_cn: Regex::new(r"^1[34578]\d{9}$").unwrap(),
        positive_number: Regex::new(r"^\d*\.?\d+$").unwrap(),
        negative_number: Regex::new(r"^-\d*\.?\d+$").unwrap(),
        url: Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap(),
        ipv4: Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap(),
        ipv6: Regex::new(r"^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$").unwrap(),
        hex_color: Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap(),
        credit_card: Regex::new(r"^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|3[47][0-9]{13}|3[0-9]{13}|6(?:011|5[0-9]{2})[0-9]{12})$").unwrap(),
        phone_us: Regex::new(r"^\+?1?[-.\s]?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})$").unwrap(),
    }
});

/// Get the global regex patterns instance
pub fn get_patterns() -> &'static RegexPatterns {
    &REGEX_PATTERNS
}

/// Validate username (4-16 characters, alphanumeric, underscore, hyphen)
pub fn is_valid_username(username: &str) -> bool {
    REGEX_PATTERNS.username.is_match(username)
}

/// Validate email address
pub fn is_valid_email(email: &str) -> bool {
    REGEX_PATTERNS.email.is_match(email)
}

/// Validate email address (supports Chinese characters)
pub fn is_valid_email_cn(email: &str) -> bool {
    REGEX_PATTERNS.email_cn.is_match(email)
}

/// Validate Chinese mobile phone number
pub fn is_valid_mobile_cn(mobile: &str) -> bool {
    REGEX_PATTERNS.mobile_cn.is_match(mobile)
}

/// Validate US phone number
pub fn is_valid_phone_us(phone: &str) -> bool {
    REGEX_PATTERNS.phone_us.is_match(phone)
}

/// Check if string is a positive number
pub fn is_positive_number(s: &str) -> bool {
    REGEX_PATTERNS.positive_number.is_match(s)
}

/// Check if string is a negative number
pub fn is_negative_number(s: &str) -> bool {
    REGEX_PATTERNS.negative_number.is_match(s)
}

/// Validate URL
pub fn is_valid_url(url: &str) -> bool {
    REGEX_PATTERNS.url.is_match(url)
}

/// Validate IPv4 address
pub fn is_valid_ipv4(ip: &str) -> bool {
    REGEX_PATTERNS.ipv4.is_match(ip)
}

/// Validate IPv6 address
pub fn is_valid_ipv6(ip: &str) -> bool {
    REGEX_PATTERNS.ipv6.is_match(ip)
}

/// Validate hex color code
pub fn is_valid_hex_color(color: &str) -> bool {
    REGEX_PATTERNS.hex_color.is_match(color)
}

/// Validate credit card number
pub fn is_valid_credit_card(card: &str) -> bool {
    REGEX_PATTERNS.credit_card.is_match(card)
}

/// Password strength analysis
#[derive(Debug, Clone)]
pub struct PasswordStrength {
    pub min_length: bool,
    pub has_lowercase: bool,
    pub has_uppercase: bool,
    pub has_digit: bool,
    pub has_special_char: bool,
    pub score: u8,
}

/// Password strength rules
static PASSWORD_RULES: Lazy<HashMap<&'static str, Regex>> = Lazy::new(|| {
    let mut rules = HashMap::new();
    rules.insert("has_lowercase", Regex::new(r"[a-z]").unwrap());
    rules.insert("has_uppercase", Regex::new(r"[A-Z]").unwrap());
    rules.insert("has_digit", Regex::new(r"\d").unwrap());
    rules.insert(
        "has_special_char",
        Regex::new(r"[!@#$%^&*()_+\-=\[\]{}|;':,.<>?]").unwrap(),
    );
    rules
});

/// Analyze password strength
pub fn analyze_password_strength(password: &str) -> PasswordStrength {
    let min_length = password.len() >= 8;
    let has_lowercase = PASSWORD_RULES["has_lowercase"].is_match(password);
    let has_uppercase = PASSWORD_RULES["has_uppercase"].is_match(password);
    let has_digit = PASSWORD_RULES["has_digit"].is_match(password);
    let has_special_char = PASSWORD_RULES["has_special_char"].is_match(password);

    let score = [
        min_length,
        has_lowercase,
        has_uppercase,
        has_digit,
        has_special_char,
    ]
    .iter()
    .map(|&b| if b { 1 } else { 0 })
    .sum();

    PasswordStrength {
        min_length,
        has_lowercase,
        has_uppercase,
        has_digit,
        has_special_char,
        score,
    }
}

/// Calculate password strength level (0-4)
pub fn calculate_password_strength_level(password: &str) -> u8 {
    let strength = analyze_password_strength(password);
    if !strength.min_length {
        0
    } else {
        strength.score - 1 // Subtract 1 because min_length is a prerequisite
    }
}

/// Extract all matches from text using a regex pattern
pub fn extract_matches(text: &str, pattern: &str) -> Result<Vec<String>, regex::Error> {
    let re = Regex::new(pattern)?;
    Ok(re.find_iter(text).map(|m| m.as_str().to_string()).collect())
}

/// Replace all matches in text using a regex pattern
pub fn replace_all_matches(
    text: &str,
    pattern: &str,
    replacement: &str,
) -> Result<String, regex::Error> {
    let re = Regex::new(pattern)?;
    Ok(re.replace_all(text, replacement).to_string())
}

/// Split text by regex pattern
pub fn split_by_pattern(text: &str, pattern: &str) -> Result<Vec<String>, regex::Error> {
    let re = Regex::new(pattern)?;
    Ok(re.split(text).map(|s| s.to_string()).collect())
}

/// Check if text matches pattern
pub fn matches_pattern(text: &str, pattern: &str) -> Result<bool, regex::Error> {
    let re = Regex::new(pattern)?;
    Ok(re.is_match(text))
}
