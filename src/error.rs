//! Error handling utilities
//!
//! This module provides custom error types and utilities for error handling.

use thiserror::Error;

/// Argument error for invalid function arguments
#[derive(Error, Debug, Clone)]
#[error("Argument error: {message}")]
pub struct ArgumentError {
    pub message: String,
}

impl ArgumentError {
    /// Create a new ArgumentError
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Validation error for data validation failures
#[derive(Error, Debug, Clone)]
#[error("Validation error for field '{field}': {message}{}", value.as_ref().map(|v| format!(" (value: '{v}')")).unwrap_or_default())]
pub struct ValidationError {
    field: String,
    message: String,
    value: Option<String>,
}

impl ValidationError {
    /// Create a new ValidationError
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            value: None,
        }
    }

    /// Create a new ValidationError with the invalid value
    pub fn with_value(
        field: impl Into<String>,
        message: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            value: Some(value.into()),
        }
    }

    /// Get the field name
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the invalid value if available
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

/// Configuration error for configuration-related issues
#[derive(Error, Debug, Clone)]
#[error("Configuration error for key '{key}': {message}")]
pub struct ConfigError {
    key: String,
    message: String,
}

impl ConfigError {
    /// Create a new ConfigError
    pub fn new(key: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            message: message.into(),
        }
    }

    /// Get the configuration key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Network error for network-related operations
#[derive(Error, Debug, Clone)]
#[error("Network error in '{operation}'{}: {message}", status_code.map(|s| format!(" (status: {s})")).unwrap_or_default())]
pub struct NetworkError {
    operation: String,
    message: String,
    status_code: Option<u16>,
}

impl NetworkError {
    /// Create a new NetworkError
    pub fn new(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            message: message.into(),
            status_code: None,
        }
    }

    /// Create a new NetworkError with status code
    pub fn with_status(
        operation: impl Into<String>,
        message: impl Into<String>,
        status_code: u16,
    ) -> Self {
        Self {
            operation: operation.into(),
            message: message.into(),
            status_code: Some(status_code),
        }
    }

    /// Get the operation name
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the status code if available
    pub fn status_code(&self) -> Option<u16> {
        self.status_code
    }
}

/// Parse error for parsing operations
#[derive(Error, Debug, Clone)]
#[error("Parse error{}: expected '{expected}', got '{input}'", position.map(|p| format!(" at position {p}")).unwrap_or_default())]
pub struct ParseError {
    input: String,
    expected: String,
    position: Option<usize>,
}

impl ParseError {
    /// Create a new ParseError
    pub fn new(input: impl Into<String>, expected: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            expected: expected.into(),
            position: None,
        }
    }

    /// Create a new ParseError with position
    pub fn with_position(
        input: impl Into<String>,
        expected: impl Into<String>,
        position: usize,
    ) -> Self {
        Self {
            input: input.into(),
            expected: expected.into(),
            position: Some(position),
        }
    }

    /// Get the input that failed to parse
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Get what was expected
    pub fn expected(&self) -> &str {
        &self.expected
    }

    /// Get the position where parsing failed
    pub fn position(&self) -> Option<usize> {
        self.position
    }
}

/// Generic utility error that can wrap other errors
#[derive(Error, Debug)]
pub enum UtilsError {
    #[error(transparent)]
    Argument(#[from] ArgumentError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error: {0}")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Result type alias for utils operations
pub type UtilsResult<T> = Result<T, UtilsError>;

/// Create an argument error
pub fn argument_error(message: impl Into<String>) -> ArgumentError {
    ArgumentError::new(message)
}

/// Create a validation error
pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> ValidationError {
    ValidationError::new(field, message)
}

/// Create a config error
pub fn config_error(key: impl Into<String>, message: impl Into<String>) -> ConfigError {
    ConfigError::new(key, message)
}

/// Create a network error
pub fn network_error(operation: impl Into<String>, message: impl Into<String>) -> NetworkError {
    NetworkError::new(operation, message)
}

/// Create a parse error
pub fn parse_error(input: impl Into<String>, expected: impl Into<String>) -> ParseError {
    ParseError::new(input, expected)
}
