//! Integration tests for logger utilities

use mudssky_utils::logger::*;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct TestOutput {
    messages: Arc<Mutex<Vec<String>>>,
}

impl TestOutput {
    fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_messages(&self) -> Vec<String> {
        self.messages.lock().unwrap().clone()
    }
}

impl LogOutput for TestOutput {
    fn write(&self, formatted_message: &str) {
        self.messages.lock().unwrap().push(formatted_message.to_string());
    }
}

#[test]
fn test_log_level_ordering() {
    assert!(LogLevel::Trace < LogLevel::Debug);
    assert!(LogLevel::Debug < LogLevel::Info);
    assert!(LogLevel::Info < LogLevel::Warn);
    assert!(LogLevel::Warn < LogLevel::Error);
}

#[test]
fn test_log_level_from_str() {
    assert_eq!("INFO".parse::<LogLevel>().unwrap(), LogLevel::Info);
    assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
    assert!("invalid".parse::<LogLevel>().is_err());
}

#[test]
fn test_simple_formatter() {
    let formatter = SimpleFormatter::default();
    let entry = LogEntry::new(
        LogLevel::Info,
        "test".to_string(),
        "Hello world".to_string(),
    );
    let formatted = formatter.format(&entry);

    assert!(formatted.contains("[INFO]"));
    assert!(formatted.contains("(test)"));
    assert!(formatted.contains("Hello world"));
}

#[test]
fn test_json_formatter() {
    let formatter = JsonFormatter;
    let entry = LogEntry::new(
        LogLevel::Error,
        "test".to_string(),
        "Error occurred".to_string(),
    );
    let formatted = formatter.format(&entry);

    let parsed: Value = serde_json::from_str(&formatted).unwrap();
    assert_eq!(parsed["level"], "ERROR");
    assert_eq!(parsed["logger"], "test");
    assert_eq!(parsed["message"], "Error occurred");
}

#[test]
fn test_logger_level_filtering() {
    let test_output = Arc::new(TestOutput::new());
    let config = LoggerConfig::new("test".to_string())
        .with_level(LogLevel::Warn)
        .with_output(test_output.clone());

    let logger = Logger::new(config);

    logger.debug("Debug message");
    logger.info("Info message");
    logger.warn("Warning message");
    logger.error("Error message");

    let messages = test_output.get_messages();
    assert_eq!(messages.len(), 2); // Only warn and error should be logged
    assert!(messages[0].contains("Warning message"));
    assert!(messages[1].contains("Error message"));
}

#[test]
fn test_logger_with_metadata() {
    let test_output = Arc::new(TestOutput::new());
    let config = LoggerConfig::new("test".to_string())
        .with_output(test_output.clone())
        .with_formatter(Arc::new(JsonFormatter));

    let logger = Logger::new(config);

    let mut metadata = HashMap::new();
    metadata.insert("user_id".to_string(), json!("12345"));
    metadata.insert("action".to_string(), json!("login"));

    logger.log_with_metadata(LogLevel::Info, "User logged in", metadata);

    let messages = test_output.get_messages();
    assert_eq!(messages.len(), 1);

    let parsed: Value = serde_json::from_str(&messages[0]).unwrap();
    assert_eq!(parsed["user_id"], "12345");
    assert_eq!(parsed["action"], "login");
}

#[test]
fn test_log_entry_with_metadata() {
    let entry = LogEntry::new(LogLevel::Info, "test".to_string(), "message".to_string())
        .with_metadata("key1".to_string(), json!("value1"))
        .with_metadata("key2".to_string(), json!(42));

    assert_eq!(entry.metadata.len(), 2);
    assert_eq!(entry.metadata["key1"], json!("value1"));
    assert_eq!(entry.metadata["key2"], json!(42));
}
