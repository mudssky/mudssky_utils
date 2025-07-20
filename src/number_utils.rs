//! Number utility functions similar to JavaScript number methods
//!
//! This module provides utility functions for working with numbers
//! that are commonly available in JavaScript but not natively in Rust.

/// Error type for number operations
#[derive(Debug, Clone, PartialEq)]
pub enum NumberUtilsError {
    /// Invalid number format
    InvalidFormat(String),
    /// Number out of range
    OutOfRange(String),
    /// Division by zero
    DivisionByZero,
}

impl std::fmt::Display for NumberUtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberUtilsError::InvalidFormat(msg) => write!(f, "Invalid format: {msg}"),
            NumberUtilsError::OutOfRange(msg) => write!(f, "Out of range: {msg}"),
            NumberUtilsError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

impl std::error::Error for NumberUtilsError {}

/// Check if a number is finite
/// Similar to JavaScript's Number.isFinite()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::is_finite;
///
/// assert!(is_finite(42.0));
/// assert!(is_finite(-42.0));
/// assert!(!is_finite(f64::INFINITY));
/// assert!(!is_finite(f64::NEG_INFINITY));
/// assert!(!is_finite(f64::NAN));
/// ```
pub fn is_finite(n: f64) -> bool {
    n.is_finite()
}

/// Check if a number is NaN
/// Similar to JavaScript's Number.isNaN()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::is_nan;
///
/// assert!(!is_nan(42.0));
/// assert!(!is_nan(f64::INFINITY));
/// assert!(is_nan(f64::NAN));
/// assert!(is_nan(0.0 / 0.0));
/// ```
pub fn is_nan(n: f64) -> bool {
    n.is_nan()
}

/// Check if a number is an integer
/// Similar to JavaScript's Number.isInteger()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::is_integer;
///
/// assert!(is_integer(42.0));
/// assert!(is_integer(-42.0));
/// assert!(is_integer(0.0));
/// assert!(!is_integer(42.5));
/// assert!(!is_integer(f64::NAN));
/// assert!(!is_integer(f64::INFINITY));
/// ```
pub fn is_integer(n: f64) -> bool {
    n.is_finite() && n.fract() == 0.0
}

/// Check if a number is a safe integer
/// Similar to JavaScript's Number.isSafeInteger()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::is_safe_integer;
///
/// assert!(is_safe_integer(42.0));
/// assert!(is_safe_integer(-42.0));
/// assert!(is_safe_integer(9007199254740991.0)); // MAX_SAFE_INTEGER
/// assert!(!is_safe_integer(9007199254740992.0)); // MAX_SAFE_INTEGER + 1
/// assert!(!is_safe_integer(42.5));
/// ```
pub fn is_safe_integer(n: f64) -> bool {
    const MAX_SAFE_INTEGER: f64 = 9007199254740991.0; // 2^53 - 1
    is_integer(n) && n.abs() <= MAX_SAFE_INTEGER
}

/// Parse a string to a float
/// Similar to JavaScript's parseFloat()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::parse_float;
///
/// assert_eq!(parse_float("42.5"), Ok(42.5));
/// assert_eq!(parse_float("42"), Ok(42.0));
/// assert_eq!(parse_float("42.5abc"), Ok(42.5));
/// assert!(parse_float("abc").is_err());
/// ```
pub fn parse_float(s: &str) -> Result<f64, NumberUtilsError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(NumberUtilsError::InvalidFormat("Empty string".to_string()));
    }

    // Find the longest valid number prefix
    let mut end_idx = 0;
    let mut has_dot = false;
    let mut has_e = false;
    let chars: Vec<char> = trimmed.chars().collect();

    // Handle optional sign
    if !chars.is_empty() && (chars[0] == '+' || chars[0] == '-') {
        end_idx = 1;
    }

    while end_idx < chars.len() {
        let ch = chars[end_idx];
        match ch {
            '0'..='9' => end_idx += 1,
            '.' if !has_dot && !has_e => {
                has_dot = true;
                end_idx += 1;
            }
            'e' | 'E' if !has_e && end_idx > 0 => {
                has_e = true;
                end_idx += 1;
                // Handle optional sign after e/E
                if end_idx < chars.len() && (chars[end_idx] == '+' || chars[end_idx] == '-') {
                    end_idx += 1;
                }
            }
            _ => break,
        }
    }

    if end_idx == 0 || (end_idx == 1 && (chars[0] == '+' || chars[0] == '-')) {
        return Err(NumberUtilsError::InvalidFormat(
            "No valid number found".to_string(),
        ));
    }

    let number_str: String = chars[0..end_idx].iter().collect();
    number_str
        .parse::<f64>()
        .map_err(|_| NumberUtilsError::InvalidFormat(format!("Cannot parse: {number_str}")))
}

/// Parse a string to an integer with specified radix
/// Similar to JavaScript's parseInt()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::parse_int;
///
/// assert_eq!(parse_int("42", 10), Ok(42));
/// assert_eq!(parse_int("101", 2), Ok(5));
/// assert_eq!(parse_int("ff", 16), Ok(255));
/// assert_eq!(parse_int("42abc", 10), Ok(42));
/// assert!(parse_int("abc", 10).is_err());
/// ```
pub fn parse_int(s: &str, radix: u32) -> Result<i64, NumberUtilsError> {
    if !(2..=36).contains(&radix) {
        return Err(NumberUtilsError::InvalidFormat(
            "Radix must be between 2 and 36".to_string(),
        ));
    }

    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(NumberUtilsError::InvalidFormat("Empty string".to_string()));
    }

    let chars: Vec<char> = trimmed.chars().collect();
    let mut start_idx = 0;
    let mut is_negative = false;

    // Handle optional sign
    if !chars.is_empty() {
        match chars[0] {
            '-' => {
                is_negative = true;
                start_idx = 1;
            }
            '+' => start_idx = 1,
            _ => {}
        }
    }

    // Find the longest valid number prefix
    let mut end_idx = start_idx;
    while end_idx < chars.len() {
        let ch = chars[end_idx];
        let digit_value = match ch {
            '0'..='9' => (ch as u32) - ('0' as u32),
            'a'..='z' => (ch as u32) - ('a' as u32) + 10,
            'A'..='Z' => (ch as u32) - ('A' as u32) + 10,
            _ => break,
        };

        if digit_value >= radix {
            break;
        }
        end_idx += 1;
    }

    if end_idx == start_idx {
        return Err(NumberUtilsError::InvalidFormat(
            "No valid digits found".to_string(),
        ));
    }

    let number_str: String = chars[start_idx..end_idx].iter().collect();
    let result = i64::from_str_radix(&number_str, radix)
        .map_err(|_| NumberUtilsError::InvalidFormat(format!("Cannot parse: {number_str}")))?;

    Ok(if is_negative { -result } else { result })
}

/// Convert number to fixed decimal places
/// Similar to JavaScript's Number.prototype.toFixed()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::to_fixed;
///
/// assert_eq!(to_fixed(42.12345, 2), "42.12");
/// assert_eq!(to_fixed(42.0, 2), "42.00");
/// assert_eq!(to_fixed(42.999, 2), "43.00");
/// ```
pub fn to_fixed(n: f64, digits: usize) -> String {
    if digits > 100 {
        return format!("{n:.100}");
    }
    format!("{n:.digits$}")
}

/// Convert number to exponential notation
/// Similar to JavaScript's Number.prototype.toExponential()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::to_exponential;
///
/// assert_eq!(to_exponential(42.0, Some(2)), "4.20e1");
/// assert_eq!(to_exponential(0.00042, Some(2)), "4.20e-4");
/// ```
pub fn to_exponential(n: f64, fraction_digits: Option<usize>) -> String {
    match fraction_digits {
        Some(digits) => {
            let digits = digits.min(100);
            format!("{n:.digits$e}")
        }
        None => format!("{n:e}"),
    }
}

/// Convert number to precision notation
/// Similar to JavaScript's Number.prototype.toPrecision()
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::to_precision;
///
/// assert_eq!(to_precision(42.12345, Some(4)), "42.12");
/// assert_eq!(to_precision(0.00042, Some(2)), "4.2e-4");
/// ```
pub fn to_precision(n: f64, precision: Option<usize>) -> String {
    match precision {
        Some(p) if p > 0 => {
            let p = p.min(100);
            if n == 0.0 {
                return "0".repeat(p);
            }

            let abs_n = n.abs();
            let log10 = abs_n.log10().floor() as i32;

            if log10 >= 0 && log10 < p as i32 {
                // Use fixed notation
                let decimal_places = (p as i32 - log10 - 1).max(0) as usize;
                format!("{n:.decimal_places$}")
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            } else {
                // Use exponential notation
                format!("{n:.precision$e}", precision = p - 1)
            }
        }
        _ => n.to_string(),
    }
}

/// Get the maximum safe integer value
/// Similar to JavaScript's Number.MAX_SAFE_INTEGER
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::max_safe_integer;
///
/// assert_eq!(max_safe_integer(), 9007199254740991.0);
/// ```
pub fn max_safe_integer() -> f64 {
    9007199254740991.0 // 2^53 - 1
}

/// Get the minimum safe integer value
/// Similar to JavaScript's Number.MIN_SAFE_INTEGER
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::min_safe_integer;
///
/// assert_eq!(min_safe_integer(), -9007199254740991.0);
/// ```
pub fn min_safe_integer() -> f64 {
    -9007199254740991.0 // -(2^53 - 1)
}

/// Get positive infinity
/// Similar to JavaScript's Number.POSITIVE_INFINITY
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::positive_infinity;
///
/// assert_eq!(positive_infinity(), f64::INFINITY);
/// ```
pub fn positive_infinity() -> f64 {
    f64::INFINITY
}

/// Get negative infinity
/// Similar to JavaScript's Number.NEGATIVE_INFINITY
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::negative_infinity;
///
/// assert_eq!(negative_infinity(), f64::NEG_INFINITY);
/// ```
pub fn negative_infinity() -> f64 {
    f64::NEG_INFINITY
}

/// Clamp a number between min and max values
/// Similar to CSS clamp() function
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::clamp;
///
/// assert_eq!(clamp(5.0, 1.0, 10.0), 5.0);
/// assert_eq!(clamp(0.0, 1.0, 10.0), 1.0);
/// assert_eq!(clamp(15.0, 1.0, 10.0), 10.0);
/// ```
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between two values
/// Common in animations and graphics
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::lerp;
///
/// assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
/// assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
/// assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
/// ```
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

/// Map a value from one range to another
/// Common in data visualization and scaling
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::number_utils::map_range;
///
/// assert_eq!(map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
/// assert_eq!(map_range(0.0, 0.0, 10.0, 0.0, 100.0), 0.0);
/// assert_eq!(map_range(10.0, 0.0, 10.0, 0.0, 100.0), 100.0);
/// ```
pub fn map_range(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
