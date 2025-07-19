//! Logging utilities
//!
//! This module provides a flexible logging system with different log levels,
//! formatters, and output targets.

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Log levels in order of severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(LogLevel::Trace),
            "DEBUG" => Ok(LogLevel::Debug),
            "INFO" => Ok(LogLevel::Info),
            "WARN" => Ok(LogLevel::Warn),
            "ERROR" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {s}")),
        }
    }
}

/// Log entry structure
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub logger_name: String,
    pub message: String,
    pub metadata: HashMap<String, Value>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(level: LogLevel, logger_name: String, message: String) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            logger_name,
            message,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the log entry
    pub fn with_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Add multiple metadata entries
    pub fn with_metadata_map(mut self, metadata: HashMap<String, Value>) -> Self {
        self.metadata.extend(metadata);
        self
    }
}

/// Log formatter trait
pub trait LogFormatter: Send + Sync {
    fn format(&self, entry: &LogEntry) -> String;
}

/// Simple text formatter
#[derive(Debug, Clone)]
pub struct SimpleFormatter {
    pub include_timestamp: bool,
    pub include_level: bool,
    pub include_logger_name: bool,
}

impl Default for SimpleFormatter {
    fn default() -> Self {
        Self {
            include_timestamp: true,
            include_level: true,
            include_logger_name: true,
        }
    }
}

impl LogFormatter for SimpleFormatter {
    fn format(&self, entry: &LogEntry) -> String {
        let mut parts = Vec::new();

        if self.include_timestamp {
            parts.push(entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string());
        }

        if self.include_level {
            parts.push(format!("[{}]", entry.level));
        }

        if self.include_logger_name {
            parts.push(format!("({})", entry.logger_name));
        }

        parts.push(entry.message.clone());

        if !entry.metadata.is_empty() {
            let metadata_str = serde_json::to_string(&entry.metadata).unwrap_or_default();
            parts.push(format!("metadata: {metadata_str}"));
        }

        parts.join(" ")
    }
}

/// JSON formatter
#[derive(Debug, Clone, Default)]
pub struct JsonFormatter;

impl LogFormatter for JsonFormatter {
    fn format(&self, entry: &LogEntry) -> String {
        let mut json_entry = json!({
            "timestamp": entry.timestamp.to_rfc3339(),
            "level": entry.level.to_string(),
            "logger": entry.logger_name,
            "message": entry.message,
        });

        if !entry.metadata.is_empty() {
            if let Value::Object(ref mut map) = json_entry {
                for (key, value) in &entry.metadata {
                    map.insert(key.clone(), value.clone());
                }
            }
        }

        serde_json::to_string(&json_entry).unwrap_or_default()
    }
}

/// Log output trait
pub trait LogOutput: Send + Sync {
    fn write(&self, formatted_message: &str);
}

/// Console output
#[derive(Debug, Clone, Default)]
pub struct ConsoleOutput;

impl LogOutput for ConsoleOutput {
    fn write(&self, formatted_message: &str) {
        println!("{formatted_message}");
    }
}

/// Logger configuration
#[derive(Clone)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub name: String,
    pub formatter: Arc<dyn LogFormatter>,
    pub output: Arc<dyn LogOutput>,
}

impl std::fmt::Debug for LoggerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoggerConfig")
            .field("level", &self.level)
            .field("name", &self.name)
            .field("formatter", &"<formatter>")
            .field("output", &"<output>")
            .finish()
    }
}

impl LoggerConfig {
    /// Create a new logger configuration
    pub fn new(name: String) -> Self {
        Self {
            name,
            level: LogLevel::Info,
            formatter: Arc::new(SimpleFormatter::default()),
            output: Arc::new(ConsoleOutput),
        }
    }

    /// Set the log level
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the formatter
    pub fn with_formatter(mut self, formatter: Arc<dyn LogFormatter>) -> Self {
        self.formatter = formatter;
        self
    }

    /// Set the output
    pub fn with_output(mut self, output: Arc<dyn LogOutput>) -> Self {
        self.output = output;
        self
    }
}

/// Logger implementation
#[derive(Debug, Clone)]
pub struct Logger {
    config: LoggerConfig,
}

impl Logger {
    /// Create a new logger
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Create a logger with default configuration
    pub fn with_name(name: &str) -> Self {
        Self::new(LoggerConfig::new(name.to_string()))
    }

    /// Check if a log level is enabled
    pub fn is_enabled(&self, level: LogLevel) -> bool {
        level >= self.config.level
    }

    /// Log a message at the specified level
    pub fn log(&self, level: LogLevel, message: &str) {
        if self.is_enabled(level) {
            let entry = LogEntry::new(level, self.config.name.clone(), message.to_string());
            let formatted = self.config.formatter.format(&entry);
            self.config.output.write(&formatted);
        }
    }

    /// Log a message with metadata
    pub fn log_with_metadata(
        &self,
        level: LogLevel,
        message: &str,
        metadata: HashMap<String, Value>,
    ) {
        if self.is_enabled(level) {
            let entry = LogEntry::new(level, self.config.name.clone(), message.to_string())
                .with_metadata_map(metadata);
            let formatted = self.config.formatter.format(&entry);
            self.config.output.write(&formatted);
        }
    }

    /// Log a trace message
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }

    /// Log a debug message
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Log an info message
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Log a warning message
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Log an error message
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

/// Global logger registry
static LOGGER_REGISTRY: Lazy<Mutex<HashMap<String, Logger>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Create or get a logger by name
pub fn get_logger(name: &str) -> Logger {
    let mut registry = LOGGER_REGISTRY.lock().unwrap();

    if let Some(logger) = registry.get(name) {
        logger.clone()
    } else {
        let logger = Logger::with_name(name);
        registry.insert(name.to_string(), logger.clone());
        logger
    }
}

/// Create a logger with custom configuration
pub fn create_logger(config: LoggerConfig) -> Logger {
    let logger = Logger::new(config.clone());
    let mut registry = LOGGER_REGISTRY.lock().unwrap();
    registry.insert(config.name.clone(), logger.clone());
    logger
}

/// Set the global log level for all loggers
pub fn set_global_level(level: LogLevel) {
    let mut registry = LOGGER_REGISTRY.lock().unwrap();
    for logger in registry.values_mut() {
        logger.config.level = level;
    }
}
