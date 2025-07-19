//! Bytes utilities module
//!
//! This module provides utilities for byte unit conversion and formatting.

use thiserror::Error;

/// Error types for bytes operations
#[derive(Error, Debug, PartialEq)]
pub enum BytesError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid unit: {0}")]
    InvalidUnit(String),
}

/// Byte unit types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteUnit {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
}

impl ByteUnit {
    /// Get the multiplier for this unit
    pub fn multiplier(&self) -> u64 {
        match self {
            ByteUnit::B => 1,
            ByteUnit::KB => 1 << 10,
            ByteUnit::MB => 1 << 20,
            ByteUnit::GB => 1 << 30,
            ByteUnit::TB => 1u64 << 40,
            ByteUnit::PB => 1u64 << 50,
        }
    }

    /// Parse unit from string
    pub fn from_str(s: &str) -> Result<Self, BytesError> {
        match s.to_lowercase().as_str() {
            "b" => Ok(ByteUnit::B),
            "kb" => Ok(ByteUnit::KB),
            "mb" => Ok(ByteUnit::MB),
            "gb" => Ok(ByteUnit::GB),
            "tb" => Ok(ByteUnit::TB),
            "pb" => Ok(ByteUnit::PB),
            _ => Err(BytesError::InvalidUnit(s.to_string())),
        }
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        match self {
            ByteUnit::B => "B".to_string(),
            ByteUnit::KB => "KB".to_string(),
            ByteUnit::MB => "MB".to_string(),
            ByteUnit::GB => "GB".to_string(),
            ByteUnit::TB => "TB".to_string(),
            ByteUnit::PB => "PB".to_string(),
        }
    }
}

/// Options for byte formatting
#[derive(Debug, Clone)]
pub struct BytesOptions {
    pub unit: Option<ByteUnit>,
    pub decimal_places: usize,
    pub fixed_decimals: bool,
    pub thousands_separator: String,
    pub unit_separator: String,
}

impl Default for BytesOptions {
    fn default() -> Self {
        Self {
            unit: None,
            decimal_places: 2,
            fixed_decimals: false,
            thousands_separator: String::new(),
            unit_separator: String::new(),
        }
    }
}

/// Bytes utility struct for conversion and formatting
pub struct Bytes;

impl Bytes {
    /// Create a new Bytes instance
    pub fn new() -> Self {
        Self
    }

    /// Convert between number and string representation
    ///
    /// # Examples
    ///
    /// ```
    /// use mudssky_utils::bytes::{Bytes, BytesOptions};
    ///
    /// let bytes = Bytes::new();
    ///
    /// // Format number to string
    /// let result = bytes.convert_number(1024, None).unwrap();
    /// assert_eq!(result, "1KB");
    ///
    /// // Parse string to number
    /// let result = bytes.convert_string("1KB").unwrap();
    /// assert_eq!(result, 1024);
    /// ```
    pub fn convert_number(
        &self,
        value: u64,
        options: Option<BytesOptions>,
    ) -> Result<String, BytesError> {
        self.format(value, options)
    }

    /// Convert string to number
    pub fn convert_string(&self, value: &str) -> Result<u64, BytesError> {
        self.parse(value)
    }

    /// Parse a byte string to number
    ///
    /// # Examples
    ///
    /// ```
    /// use mudssky_utils::bytes::Bytes;
    ///
    /// let bytes = Bytes::new();
    /// assert_eq!(bytes.parse("1KB").unwrap(), 1024);
    /// assert_eq!(bytes.parse("1.5MB").unwrap(), 1572864);
    /// assert_eq!(bytes.parse("100").unwrap(), 100);
    /// ```
    pub fn parse(&self, val: &str) -> Result<u64, BytesError> {
        let val = val.trim();

        // Try to parse as plain number first
        if let Ok(num) = val.parse::<f64>() {
            if num < 0.0 {
                return Err(BytesError::ParseError(
                    "Negative values not allowed".to_string(),
                ));
            }
            return Ok(num.floor() as u64);
        }

        // Use regex-like parsing
        let re = regex::Regex::new(r"^([-+]?\d+(?:\.\d+)?)\s*(b|kb|mb|gb|tb|pb)?$")
            .map_err(|e| BytesError::ParseError(format!("Regex error: {}", e)))?;

        if let Some(captures) = re.captures(&val.to_lowercase()) {
            let number_str = captures.get(1).unwrap().as_str();
            let unit_str = captures.get(2).map(|m| m.as_str()).unwrap_or("b");

            let float_value: f64 = number_str
                .parse()
                .map_err(|_| BytesError::ParseError(format!("Invalid number: {}", number_str)))?;

            if float_value < 0.0 {
                return Err(BytesError::ParseError(
                    "Negative values not allowed".to_string(),
                ));
            }

            let unit = ByteUnit::from_str(unit_str)?;
            let multiplier = unit.multiplier();

            Ok((float_value * multiplier as f64).floor() as u64)
        } else {
            Err(BytesError::ParseError(format!("Invalid format: {}", val)))
        }
    }

    /// Format a number as byte string
    ///
    /// # Examples
    ///
    /// ```
    /// use mudssky_utils::bytes::{Bytes, BytesOptions, ByteUnit};
    ///
    /// let bytes = Bytes::new();
    ///
    /// // Auto unit selection
    /// assert_eq!(bytes.format(1024, None).unwrap(), "1KB");
    /// assert_eq!(bytes.format(1536, None).unwrap(), "1.5KB");
    ///
    /// // Custom options
    /// let mut options = BytesOptions::default();
    /// options.unit = Some(ByteUnit::MB);
    /// options.decimal_places = 3;
    /// options.fixed_decimals = true;
    /// assert_eq!(bytes.format(1048576, Some(options)).unwrap(), "1.000MB");
    /// ```
    pub fn format(&self, value: u64, options: Option<BytesOptions>) -> Result<String, BytesError> {
        let options = options.unwrap_or_default();

        let num = value as f64;
        let unit = if let Some(unit) = options.unit {
            unit
        } else {
            // Auto-select unit
            if num >= ByteUnit::PB.multiplier() as f64 {
                ByteUnit::PB
            } else if num >= ByteUnit::TB.multiplier() as f64 {
                ByteUnit::TB
            } else if num >= ByteUnit::GB.multiplier() as f64 {
                ByteUnit::GB
            } else if num >= ByteUnit::MB.multiplier() as f64 {
                ByteUnit::MB
            } else if num >= ByteUnit::KB.multiplier() as f64 {
                ByteUnit::KB
            } else {
                ByteUnit::B
            }
        };

        let val = num / unit.multiplier() as f64;
        let mut num_str = format!("{:.prec$}", val, prec = options.decimal_places);

        if !options.fixed_decimals {
            // Remove trailing zeros
            if num_str.contains('.') {
                num_str = num_str.trim_end_matches('0').trim_end_matches('.').to_string();
            }
        }

        if !options.thousands_separator.is_empty() {
            num_str = self.add_thousands_separator(&num_str, &options.thousands_separator);
        }

        Ok(format!(
            "{}{}{}",
            num_str,
            options.unit_separator,
            unit.to_string()
        ))
    }

    /// Add thousands separator to a number string
    fn add_thousands_separator(&self, num_str: &str, separator: &str) -> String {
        let parts: Vec<&str> = num_str.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = if parts.len() > 1 {
            Some(parts[1])
        } else {
            None
        };

        let mut result = String::new();
        let chars: Vec<char> = integer_part.chars().rev().collect();

        for (i, ch) in chars.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push_str(separator);
            }
            result.push(*ch);
        }

        let integer_result: String = result.chars().rev().collect();

        if let Some(decimal) = decimal_part {
            format!("{}.{}", integer_result, decimal)
        } else {
            integer_result
        }
    }
}

impl Default for Bytes {
    fn default() -> Self {
        Self::new()
    }
}

/// Global bytes instance
static BYTES_INSTANCE: std::sync::OnceLock<Bytes> = std::sync::OnceLock::new();

fn get_bytes_instance() -> &'static Bytes {
    BYTES_INSTANCE.get_or_init(|| Bytes::new())
}

/// Convenience function for byte conversion
///
/// # Examples
///
/// ```
/// use mudssky_utils::bytes::bytes;
///
/// // Format number to string
/// assert_eq!(bytes(1024).unwrap(), "1KB");
/// ```
pub fn bytes(value: u64) -> Result<String, BytesError> {
    get_bytes_instance().convert_number(value, None)
}

/// Parse byte string to number
///
/// # Examples
///
/// ```
/// use mudssky_utils::bytes::parse_bytes;
///
/// assert_eq!(parse_bytes("1KB").unwrap(), 1024);
/// assert_eq!(parse_bytes("1.5MB").unwrap(), 1572864);
/// ```
pub fn parse_bytes(value: &str) -> Result<u64, BytesError> {
    get_bytes_instance().convert_string(value)
}
