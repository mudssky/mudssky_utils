//! String utility functions for various string operations
//!
//! This module provides a comprehensive set of string manipulation utilities including:
//! - Case conversion (camelCase, snake_case, PascalCase, dash-case)
//! - String trimming with custom characters
//! - Template parsing and substitution
//! - Random string generation
//! - File extension extraction
//! - Fuzzy matching
//!
//! # Examples
//!
//! ```rust
//! use mudssky_utils::string::*;
//!
//! // Case conversion
//! assert_eq!(camel_case("hello world"), "helloWorld");
//! assert_eq!(snake_case("helloWorld"), "hello_world");
//!
//! // String trimming
//! assert_eq!(trim("  hello  ", None), "hello");
//! assert_eq!(trim("__hello__", Some("_")), "hello");
//!
//! // Template parsing
//! let template = "Hello {{name}}, welcome to {{place}}!";
//! let mut data = std::collections::HashMap::new();
//! data.insert("name".to_string(), "World".to_string());
//! data.insert("place".to_string(), "Rust".to_string());
//! assert_eq!(parse_template(template, &data, None), "Hello World, welcome to Rust!");
//! ```

use regex::Regex;
use std::collections::HashMap;
use thiserror::Error;

/// Split a string into words based on common delimiters and camelCase boundaries
fn split_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() || ch == '.' || ch == '-' || ch == '_' {
            if !current_word.is_empty() {
                words.push(current_word.clone());
                current_word.clear();
            }
        } else if ch.is_uppercase() && !current_word.is_empty() {
            // Check if this is a camelCase boundary
            if let Some(&next_ch) = chars.peek() {
                if next_ch.is_lowercase() {
                    // This is a camelCase boundary
                    words.push(current_word.clone());
                    current_word.clear();
                }
            }
            current_word.push(ch);
        } else {
            current_word.push(ch);
        }
    }

    if !current_word.is_empty() {
        words.push(current_word);
    }

    words
}

/// Errors that can occur during string operations
#[derive(Error, Debug, PartialEq)]
pub enum StringError {
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    #[error("Regex error: {message}")]
    RegexError { message: String },
}

/// Generate all possible case combinations for letters in a string
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::gen_all_cases_combination;
///
/// let result = gen_all_cases_combination("mb");
/// assert_eq!(result, vec!["mb", "mB", "Mb", "MB"]);
/// ```
pub fn gen_all_cases_combination(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = s.chars().collect();

    fn dfs(chars: &[char], index: usize, current: &mut String, result: &mut Vec<String>) {
        if index == chars.len() {
            result.push(current.clone());
            return;
        }

        let ch = chars[index];
        if ch.is_alphabetic() {
            // Try lowercase
            current.push(ch.to_lowercase().next().unwrap_or(ch));
            dfs(chars, index + 1, current, result);
            current.pop();

            // Try uppercase
            current.push(ch.to_uppercase().next().unwrap_or(ch));
            dfs(chars, index + 1, current, result);
            current.pop();
        } else {
            // Non-alphabetic character
            current.push(ch);
            dfs(chars, index + 1, current, result);
            current.pop();
        }
    }

    let mut current = String::new();
    dfs(&chars, 0, &mut current, &mut result);
    result
}

/// Generate a UUID v4 string
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::generate_uuid;
///
/// let uuid = generate_uuid();
/// assert_eq!(uuid.len(), 36);
/// assert!(uuid.contains('-'));
/// ```
pub fn generate_uuid() -> String {
    use std::fmt::Write;

    let mut uuid = String::with_capacity(36);
    let rng = || -> u8 { (rand::random::<f64>() * 16.0) as u8 };

    for i in 0..32 {
        if i == 8 || i == 12 || i == 16 || i == 20 {
            uuid.push('-');
        }

        let digit = if i == 12 {
            4 // Version 4
        } else if i == 16 {
            (rng() & 0x3) | 0x8 // Variant bits
        } else {
            rng()
        };

        write!(&mut uuid, "{digit:x}").unwrap();
    }

    uuid
}

const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Generate a random base62 string of specified length
///
/// # Arguments
///
/// * `len` - Length of the generated string
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::generate_base62_code;
///
/// let code = generate_base62_code(6).unwrap();
/// assert_eq!(code.len(), 6);
/// ```
///
/// # Errors
///
/// Returns `StringError::InvalidInput` if length is 0
pub fn generate_base62_code(len: usize) -> Result<String, StringError> {
    if len == 0 {
        return Err(StringError::InvalidInput {
            message: "Length must be greater than 0".to_string(),
        });
    }

    let mut result = String::with_capacity(len);
    for _ in 0..len {
        let idx = (rand::random::<f64>() * 62.0) as usize;
        result.push(BASE62_CHARS[idx] as char);
    }

    Ok(result)
}

/// Perform fuzzy matching on strings (case-insensitive)
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::fuzzy_match;
///
/// assert!(fuzzy_match("hello", "Hello World"));
/// assert!(!fuzzy_match("xyz", "Hello World"));
/// ```
pub fn fuzzy_match(search_value: &str, target_string: &str) -> bool {
    target_string.to_lowercase().contains(&search_value.to_lowercase())
}

/// Extract file extension from filename
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::get_file_ext;
///
/// assert_eq!(get_file_ext("file.txt"), "txt");
/// assert_eq!(get_file_ext("archive.tar.gz"), "gz");
/// assert_eq!(get_file_ext("noext"), "");
/// ```
pub fn get_file_ext(filename: &str) -> &str {
    filename.rfind('.').map_or("", |pos| &filename[pos + 1..])
}

/// Capitalize the first character of a string
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::capitalize;
///
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize("va va voom"), "Va va voom");
/// assert_eq!(capitalize(""), "");
/// ```
pub fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    }
}

/// Convert string to camelCase
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::camel_case;
///
/// assert_eq!(camel_case("hello world"), "helloWorld");
/// assert_eq!(camel_case("va va-VOOM"), "vaVaVoom");
/// assert_eq!(camel_case("helloWorld"), "helloWorld");
/// ```
pub fn camel_case(s: &str) -> String {
    let parts = split_words(s);

    if parts.is_empty() {
        return String::new();
    }

    let mut result = parts[0].to_lowercase();
    for part in &parts[1..] {
        result.push_str(&capitalize(part));
    }

    result
}

/// Convert string to snake_case
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::snake_case;
///
/// assert_eq!(snake_case("hello world"), "hello_world");
/// assert_eq!(snake_case("va va-VOOM"), "va_va_voom");
/// assert_eq!(snake_case("helloWorld"), "hello_world");
/// ```
pub fn snake_case(s: &str) -> String {
    let parts = split_words(s);

    if parts.is_empty() {
        return String::new();
    }

    parts.iter().map(|part| part.to_lowercase()).collect::<Vec<_>>().join("_")
}

/// Convert string to dash-case
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::dash_case;
///
/// assert_eq!(dash_case("hello world"), "hello-world");
/// assert_eq!(dash_case("va va_VOOM"), "va-va-voom");
/// assert_eq!(dash_case("helloWorld"), "hello-world");
/// ```
pub fn dash_case(s: &str) -> String {
    let parts = split_words(s);

    if parts.is_empty() {
        return String::new();
    }

    parts.iter().map(|part| part.to_lowercase()).collect::<Vec<_>>().join("-")
}

/// Convert string to PascalCase
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::pascal_case;
///
/// assert_eq!(pascal_case("hello world"), "HelloWorld");
/// assert_eq!(pascal_case("va va boom"), "VaVaBoom");
/// ```
pub fn pascal_case(s: &str) -> String {
    let parts = split_words(s);

    parts.iter().map(|part| capitalize(part)).collect::<Vec<_>>().join("")
}

/// Parse template string and replace placeholders with data
///
/// # Arguments
///
/// * `template` - Template string with placeholders
/// * `data` - HashMap containing replacement values
/// * `regex_pattern` - Optional custom regex pattern (defaults to `{{placeholder}}`)
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::parse_template;
/// use std::collections::HashMap;
///
/// let template = "Hello {{name}}, welcome to {{place}}!";
/// let mut data = HashMap::new();
/// data.insert("name".to_string(), "World".to_string());
/// data.insert("place".to_string(), "our app".to_string());
///
/// let result = parse_template(template, &data, None);
/// assert_eq!(result, "Hello World, welcome to our app!");
/// ```
pub fn parse_template(
    template: &str,
    data: &HashMap<String, String>,
    regex_pattern: Option<&str>,
) -> String {
    let pattern = regex_pattern.unwrap_or(r"\{\{(.+?)\}\}");
    let re = Regex::new(pattern).unwrap();

    let mut result = template.to_string();
    for caps in re.captures_iter(template) {
        let full_match = caps.get(0).unwrap().as_str();
        let key = caps.get(1).unwrap().as_str();
        if let Some(value) = data.get(key) {
            result = result.replace(full_match, value);
        }
    }
    result
}

/// Trim specified characters from both ends of a string
///
/// # Arguments
///
/// * `s` - Input string
/// * `chars_to_trim` - Optional characters to trim (defaults to whitespace)
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::trim;
///
/// assert_eq!(trim("  hello world  ", None), "hello world");
/// assert_eq!(trim("__hello__", Some("_")), "hello");
/// assert_eq!(trim("-!-hello-!-", Some("-!")), "hello");
/// ```
pub fn trim(s: &str, chars_to_trim: Option<&str>) -> String {
    let chars = chars_to_trim.unwrap_or(" ");
    s.trim_matches(|c: char| chars.contains(c)).to_string()
}

/// Trim specified characters from the start of a string
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::trim_start;
///
/// assert_eq!(trim_start("  hello world  ", None), "hello world  ");
/// assert_eq!(trim_start("__hello__", Some("_")), "hello__");
/// ```
pub fn trim_start(s: &str, chars_to_trim: Option<&str>) -> String {
    let chars = chars_to_trim.unwrap_or(" ");
    s.trim_start_matches(|c: char| chars.contains(c)).to_string()
}

/// Trim specified characters from the end of a string
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::trim_end;
///
/// assert_eq!(trim_end("  hello world  ", None), "  hello world");
/// assert_eq!(trim_end("__hello__", Some("_")), "__hello");
/// ```
pub fn trim_end(s: &str, chars_to_trim: Option<&str>) -> String {
    let chars = chars_to_trim.unwrap_or(" ");
    s.trim_end_matches(|c: char| chars.contains(c)).to_string()
}

/// Remove prefix from string if it exists
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::remove_prefix;
///
/// assert_eq!(remove_prefix("hello world", "hello "), "world");
/// assert_eq!(remove_prefix("__hello__", "__"), "hello__");
/// assert_eq!(remove_prefix("test", "no"), "test");
/// ```
pub fn remove_prefix(s: &str, prefix: &str) -> String {
    if let Some(stripped) = s.strip_prefix(prefix) {
        stripped.to_string()
    } else {
        s.to_string()
    }
}

/// Generate merge paths from a list of branches
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::string::generate_merge_paths;
///
/// let branches = vec!["dev-xxx".to_string(), "dev".to_string(), "test".to_string()];
/// let paths = generate_merge_paths(&branches);
/// assert_eq!(paths, vec![
///     vec!["dev-xxx".to_string(), "dev".to_string()],
///     vec!["dev".to_string(), "test".to_string()]
/// ]);
/// ```
pub fn generate_merge_paths(branches: &[String]) -> Vec<Vec<String>> {
    if branches.len() < 2 {
        return Vec::new();
    }

    let mut paths = Vec::new();
    for i in 0..branches.len() - 1 {
        paths.push(vec![branches[i].clone(), branches[i + 1].clone()]);
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_all_cases_combination() {
        let result = gen_all_cases_combination("mb");
        assert_eq!(result, vec!["mb", "mB", "Mb", "MB"]);
    }

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_generate_base62_code() {
        let code = generate_base62_code(6).unwrap();
        assert_eq!(code.len(), 6);

        assert!(generate_base62_code(0).is_err());
    }

    #[test]
    fn test_fuzzy_match() {
        assert!(fuzzy_match("hello", "Hello World"));
        assert!(!fuzzy_match("xyz", "Hello World"));
    }

    #[test]
    fn test_get_file_ext() {
        assert_eq!(get_file_ext("file.txt"), "txt");
        assert_eq!(get_file_ext("archive.tar.gz"), "gz");
        assert_eq!(get_file_ext("noext"), "");
    }

    #[test]
    fn test_case_conversions() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(camel_case("hello world"), "helloWorld");
        assert_eq!(snake_case("helloWorld"), "hello_world");
        assert_eq!(dash_case("hello world"), "hello-world");
        assert_eq!(pascal_case("hello world"), "HelloWorld");
    }

    #[test]
    fn test_parse_template() {
        let template = "Hello {{name}}, welcome to {{place}}!";
        let mut data = HashMap::new();
        data.insert("name".to_string(), "World".to_string());
        data.insert("place".to_string(), "our app".to_string());

        let result = parse_template(template, &data, None);
        assert_eq!(result, "Hello World, welcome to our app!");
    }

    #[test]
    fn test_trim_functions() {
        assert_eq!(trim("  hello world  ", None), "hello world");
        assert_eq!(trim("__hello__", Some("_")), "hello");
        assert_eq!(trim_start("  hello world  ", None), "hello world  ");
        assert_eq!(trim_end("  hello world  ", None), "  hello world");
    }

    #[test]
    fn test_remove_prefix() {
        assert_eq!(remove_prefix("hello world", "hello "), "world");
        assert_eq!(remove_prefix("test", "no"), "test");
    }

    #[test]
    fn test_generate_merge_paths() {
        let branches = vec!["dev-xxx".to_string(), "dev".to_string(), "test".to_string()];
        let paths = generate_merge_paths(&branches);
        assert_eq!(
            paths,
            vec![
                vec!["dev-xxx".to_string(), "dev".to_string()],
                vec!["dev".to_string(), "test".to_string()]
            ]
        );
    }
}
