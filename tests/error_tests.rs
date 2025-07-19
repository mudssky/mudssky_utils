//! Integration tests for error handling utilities

use mudssky_utils::error::*;

#[test]
fn test_argument_error() {
    let err = ArgumentError::new("Invalid argument");
    assert_eq!(err.to_string(), "Argument error: Invalid argument");
}

#[test]
fn test_validation_error() {
    let err = ValidationError::new("email", "Invalid email format");
    assert_eq!(err.field(), "email");
    assert_eq!(err.message(), "Invalid email format");
    assert!(err.to_string().contains("email"));

    let err_with_value = ValidationError::with_value("age", "Must be positive", "-5");
    assert_eq!(err_with_value.value(), Some("-5"));
    assert!(err_with_value.to_string().contains("value: '-5'"));
}

#[test]
fn test_config_error() {
    let err = ConfigError::new("database.url", "Missing required configuration");
    assert_eq!(err.key(), "database.url");
    assert_eq!(err.message(), "Missing required configuration");
    assert!(err.to_string().contains("database.url"));
}

#[test]
fn test_network_error() {
    let err = NetworkError::new("fetch", "Connection timeout");
    assert_eq!(err.operation(), "fetch");
    assert_eq!(err.message(), "Connection timeout");
    assert_eq!(err.status_code(), None);
    assert!(err.to_string().contains("fetch"));

    let err_with_status = NetworkError::with_status("post", "Bad request", 400);
    assert_eq!(err_with_status.status_code(), Some(400));
    assert!(err_with_status.to_string().contains("status: 400"));
}

#[test]
fn test_parse_error() {
    let err = ParseError::new("abc", "number");
    assert_eq!(err.input(), "abc");
    assert_eq!(err.expected(), "number");
    assert_eq!(err.position(), None);
    assert!(err.to_string().contains("abc"));

    let err_with_pos = ParseError::with_position("12a", "digit", 2);
    assert_eq!(err_with_pos.position(), Some(2));
    assert!(err_with_pos.to_string().contains("position 2"));
}

#[test]
fn test_utils_error() {
    let arg_err = ArgumentError::new("test");
    let utils_err: UtilsError = arg_err.into();

    match utils_err {
        UtilsError::Argument(_) => {}
        _ => panic!("Expected ArgumentError"),
    }
}

#[test]
fn test_error_helpers() {
    let arg_err = argument_error("test");
    assert_eq!(arg_err.to_string(), "Argument error: test");

    let val_err = validation_error("field", "message");
    assert_eq!(val_err.field(), "field");

    let cfg_err = config_error("key", "message");
    assert_eq!(cfg_err.key(), "key");

    let net_err = network_error("op", "message");
    assert_eq!(net_err.operation(), "op");

    let parse_err = parse_error("input", "expected");
    assert_eq!(parse_err.input(), "input");
}

#[test]
fn test_error_chain() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let utils_err: UtilsError = io_err.into();

    match utils_err {
        UtilsError::Io(_) => {}
        _ => panic!("Expected IO error"),
    }
}

#[test]
fn test_error_cloning() {
    let arg_err = ArgumentError::new("test");
    let cloned = arg_err.clone();
    assert_eq!(arg_err.message, cloned.message);

    let val_err = ValidationError::with_value("field", "message", "value");
    let cloned = val_err.clone();
    assert_eq!(val_err.field(), cloned.field());
    assert_eq!(val_err.value(), cloned.value());
}
