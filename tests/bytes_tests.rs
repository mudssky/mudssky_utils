use mudssky_utils::bytes::*;

#[test]
fn test_bytes_parse_basic() {
    let bytes = Bytes::new();

    assert_eq!(bytes.parse("1KB").unwrap(), 1024);
    assert_eq!(bytes.parse("1MB").unwrap(), 1048576);
    assert_eq!(bytes.parse("1GB").unwrap(), 1073741824);
    assert_eq!(bytes.parse("1TB").unwrap(), 1099511627776);
}

#[test]
fn test_bytes_parse_decimal() {
    let bytes = Bytes::new();

    assert_eq!(bytes.parse("1.5KB").unwrap(), 1536);
    assert_eq!(bytes.parse("2.5MB").unwrap(), 2621440);
    assert_eq!(bytes.parse("0.5GB").unwrap(), 536870912);
}

#[test]
fn test_bytes_parse_plain_number() {
    let bytes = Bytes::new();

    assert_eq!(bytes.parse("100").unwrap(), 100);
    assert_eq!(bytes.parse("1024").unwrap(), 1024);
    assert_eq!(bytes.parse("0").unwrap(), 0);
}

#[test]
fn test_bytes_parse_case_insensitive() {
    let bytes = Bytes::new();

    assert_eq!(bytes.parse("1kb").unwrap(), 1024);
    assert_eq!(bytes.parse("1Mb").unwrap(), 1048576);
    assert_eq!(bytes.parse("1gB").unwrap(), 1073741824);
}

#[test]
fn test_bytes_parse_with_spaces() {
    let bytes = Bytes::new();

    assert_eq!(bytes.parse(" 1KB ").unwrap(), 1024);
    assert_eq!(bytes.parse("1 MB").unwrap(), 1048576);
}

#[test]
fn test_bytes_parse_errors() {
    let bytes = Bytes::new();

    assert!(bytes.parse("-1KB").is_err());
    assert!(bytes.parse("invalid").is_err());
    assert!(bytes.parse("1XB").is_err());
    assert!(bytes.parse("").is_err());
}

#[test]
fn test_bytes_format_basic() {
    let bytes = Bytes::new();

    assert_eq!(bytes.format(1024, None).unwrap(), "1KB");
    assert_eq!(bytes.format(1048576, None).unwrap(), "1MB");
    assert_eq!(bytes.format(1073741824, None).unwrap(), "1GB");
    assert_eq!(bytes.format(1099511627776, None).unwrap(), "1TB");
}

#[test]
fn test_bytes_format_decimal() {
    let bytes = Bytes::new();

    assert_eq!(bytes.format(1536, None).unwrap(), "1.5KB");
    assert_eq!(bytes.format(2621440, None).unwrap(), "2.5MB");
    assert_eq!(bytes.format(536870912, None).unwrap(), "512MB");
}

#[test]
fn test_bytes_format_small_values() {
    let bytes = Bytes::new();

    assert_eq!(bytes.format(0, None).unwrap(), "0B");
    assert_eq!(bytes.format(100, None).unwrap(), "100B");
    assert_eq!(bytes.format(512, None).unwrap(), "512B");
}

#[test]
fn test_bytes_format_with_options() {
    let bytes = Bytes::new();

    let mut options = BytesOptions::default();
    options.unit = Some(ByteUnit::MB);
    options.decimal_places = 3;
    options.fixed_decimals = true;

    assert_eq!(bytes.format(1048576, Some(options)).unwrap(), "1.000MB");
}

#[test]
fn test_bytes_format_with_unit_separator() {
    let bytes = Bytes::new();

    let mut options = BytesOptions::default();
    options.unit_separator = " ".to_string();

    assert_eq!(bytes.format(1024, Some(options)).unwrap(), "1 KB");
}

#[test]
fn test_bytes_format_with_thousands_separator() {
    let bytes = Bytes::new();

    let mut options = BytesOptions::default();
    options.thousands_separator = ",".to_string();
    options.unit = Some(ByteUnit::B);

    assert_eq!(bytes.format(1234567, Some(options)).unwrap(), "1,234,567B");
}

#[test]
fn test_bytes_convert_number() {
    let bytes = Bytes::new();

    assert_eq!(bytes.convert_number(1024, None).unwrap(), "1KB");
    assert_eq!(bytes.convert_number(1048576, None).unwrap(), "1MB");
}

#[test]
fn test_bytes_convert_string() {
    let bytes = Bytes::new();

    assert_eq!(bytes.convert_string("1KB").unwrap(), 1024);
    assert_eq!(bytes.convert_string("1MB").unwrap(), 1048576);
}

#[test]
fn test_byte_unit_multiplier() {
    assert_eq!(ByteUnit::B.multiplier(), 1);
    assert_eq!(ByteUnit::KB.multiplier(), 1024);
    assert_eq!(ByteUnit::MB.multiplier(), 1048576);
    assert_eq!(ByteUnit::GB.multiplier(), 1073741824);
    assert_eq!(ByteUnit::TB.multiplier(), 1099511627776);
    assert_eq!(ByteUnit::PB.multiplier(), 1125899906842624);
}

#[test]
fn test_byte_unit_from_str() {
    assert_eq!(ByteUnit::from_str("b").unwrap(), ByteUnit::B);
    assert_eq!(ByteUnit::from_str("KB").unwrap(), ByteUnit::KB);
    assert_eq!(ByteUnit::from_str("mb").unwrap(), ByteUnit::MB);
    assert_eq!(ByteUnit::from_str("Gb").unwrap(), ByteUnit::GB);

    assert!(ByteUnit::from_str("invalid").is_err());
}

#[test]
fn test_byte_unit_to_string() {
    assert_eq!(ByteUnit::B.to_string(), "B");
    assert_eq!(ByteUnit::KB.to_string(), "KB");
    assert_eq!(ByteUnit::MB.to_string(), "MB");
    assert_eq!(ByteUnit::GB.to_string(), "GB");
    assert_eq!(ByteUnit::TB.to_string(), "TB");
    assert_eq!(ByteUnit::PB.to_string(), "PB");
}

#[test]
fn test_convenience_functions() {
    assert_eq!(bytes(1024).unwrap(), "1KB");
    assert_eq!(bytes(1048576).unwrap(), "1MB");

    assert_eq!(parse_bytes("1KB").unwrap(), 1024);
    assert_eq!(parse_bytes("1MB").unwrap(), 1048576);
}

#[test]
fn test_round_trip_conversion() {
    let bytes_instance = Bytes::new();
    let test_values = vec![0, 100, 1024, 1536, 1048576, 2621440, 1073741824];

    for value in test_values {
        let formatted = bytes_instance.format(value, None).unwrap();
        let parsed = bytes_instance.parse(&formatted).unwrap();
        assert_eq!(parsed, value, "Round trip failed for value: {}", value);
    }
}
