//! Integration tests for the string module

use mudssky_utils::string::*;
use std::collections::HashMap;

#[test]
fn test_gen_all_cases_combination() {
    let result = gen_all_cases_combination("mb");
    assert_eq!(result, vec!["mb", "mB", "Mb", "MB"]);
    
    let result = gen_all_cases_combination("a1b");
    assert!(result.contains(&"a1b".to_string()));
    assert!(result.contains(&"A1B".to_string()));
    assert!(result.contains(&"a1B".to_string()));
    assert!(result.contains(&"A1b".to_string()));
    
    let result = gen_all_cases_combination("");
    assert_eq!(result, vec![""]);
}

#[test]
fn test_generate_uuid() {
    let uuid = generate_uuid();
    assert_eq!(uuid.len(), 36);
    assert!(uuid.contains('-'));
    
    // Check format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    let parts: Vec<&str> = uuid.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0].len(), 8);
    assert_eq!(parts[1].len(), 4);
    assert_eq!(parts[2].len(), 4);
    assert_eq!(parts[3].len(), 4);
    assert_eq!(parts[4].len(), 12);
    
    // Check version 4
    assert!(parts[2].starts_with('4'));
    
    // Generate multiple UUIDs to ensure they're different
    let uuid2 = generate_uuid();
    assert_ne!(uuid, uuid2);
}

#[test]
fn test_generate_base62_code() {
    let code = generate_base62_code(6).unwrap();
    assert_eq!(code.len(), 6);
    
    // Check all characters are valid base62
    for ch in code.chars() {
        assert!(ch.is_ascii_alphanumeric());
    }
    
    let code10 = generate_base62_code(10).unwrap();
    assert_eq!(code10.len(), 10);
    
    // Test error case
    assert!(generate_base62_code(0).is_err());
    
    // Generate multiple codes to ensure they're different
    let code2 = generate_base62_code(6).unwrap();
    assert_ne!(code, code2);
}

#[test]
fn test_fuzzy_match() {
    assert!(fuzzy_match("hello", "Hello World"));
    assert!(fuzzy_match("HELLO", "hello world"));
    assert!(fuzzy_match("world", "Hello World"));
    assert!(!fuzzy_match("xyz", "Hello World"));
    assert!(fuzzy_match("", "Hello World"));
    assert!(fuzzy_match("hello", "hello"));
}

#[test]
fn test_get_file_ext() {
    assert_eq!(get_file_ext("file.txt"), "txt");
    assert_eq!(get_file_ext("archive.tar.gz"), "gz");
    assert_eq!(get_file_ext("noext"), "");
    assert_eq!(get_file_ext(".hidden"), "hidden");
    assert_eq!(get_file_ext("file."), "");
    assert_eq!(get_file_ext("path/to/file.pdf"), "pdf");
}

#[test]
fn test_capitalize() {
    assert_eq!(capitalize("hello"), "Hello");
    assert_eq!(capitalize("va va voom"), "Va va voom");
    assert_eq!(capitalize(""), "");
    assert_eq!(capitalize("HELLO"), "Hello");
    assert_eq!(capitalize("h"), "H");
    assert_eq!(capitalize("123abc"), "123abc");
}

#[test]
fn test_camel_case() {
    assert_eq!(camel_case("hello world"), "helloWorld");
    assert_eq!(camel_case("va va-VOOM"), "vaVaVoom");
    assert_eq!(camel_case("helloWorld"), "helloWorld");
    assert_eq!(camel_case("hello_world"), "helloWorld");
    assert_eq!(camel_case("hello.world"), "helloWorld");
    assert_eq!(camel_case(""), "");
    assert_eq!(camel_case("hello"), "hello");
}

#[test]
fn test_snake_case() {
    assert_eq!(snake_case("hello world"), "hello_world");
    assert_eq!(snake_case("va va-VOOM"), "va_va_voom");
    assert_eq!(snake_case("helloWorld"), "hello_world");
    assert_eq!(snake_case("hello_world"), "hello_world");
    assert_eq!(snake_case("hello.world"), "hello_world");
    assert_eq!(snake_case(""), "");
    assert_eq!(snake_case("hello"), "hello");
}

#[test]
fn test_dash_case() {
    assert_eq!(dash_case("hello world"), "hello-world");
    assert_eq!(dash_case("va va_VOOM"), "va-va-voom");
    assert_eq!(dash_case("helloWorld"), "hello-world");
    assert_eq!(dash_case("hello-world"), "hello-world");
    assert_eq!(dash_case("hello.world"), "hello-world");
    assert_eq!(dash_case(""), "");
    assert_eq!(dash_case("hello"), "hello");
}

#[test]
fn test_pascal_case() {
    assert_eq!(pascal_case("hello world"), "HelloWorld");
    assert_eq!(pascal_case("va va boom"), "VaVaBoom");
    assert_eq!(pascal_case("hello_world"), "HelloWorld");
    assert_eq!(pascal_case("hello-world"), "HelloWorld");
    assert_eq!(pascal_case("hello.world"), "HelloWorld");
    assert_eq!(pascal_case(""), "");
    assert_eq!(pascal_case("hello"), "Hello");
}

#[test]
fn test_parse_template() {
    let template = "Hello {{name}}, welcome to {{place}}!";
    let mut data = HashMap::new();
    data.insert("name".to_string(), "World".to_string());
    data.insert("place".to_string(), "our app".to_string());
    
    let result = parse_template(template, &data, None);
    assert_eq!(result, "Hello World, welcome to our app!");
    
    // Test with missing key
    let template2 = "Hello {{name}}, welcome to {{missing}}!";
    let result2 = parse_template(template2, &data, None);
    assert_eq!(result2, "Hello World, welcome to {{missing}}!");
    
    // Test with custom regex
    let template3 = "Hi <user>, your id is <id>.";
    let mut data3 = HashMap::new();
    data3.insert("user".to_string(), "Alex".to_string());
    data3.insert("id".to_string(), "123".to_string());
    let result3 = parse_template(template3, &data3, Some(r"<(.+?)>"));
    assert_eq!(result3, "Hi Alex, your id is 123.");
    
    // Test empty template
    let result4 = parse_template("", &data, None);
    assert_eq!(result4, "");
}

#[test]
fn test_trim() {
    assert_eq!(trim("  hello world  ", None), "hello world");
    assert_eq!(trim("__hello__", Some("_")), "hello");
    assert_eq!(trim("-!-hello-!-", Some("-!")), "hello");
    assert_eq!(trim("/path/to/file/", Some("/")), "path/to/file");
    assert_eq!(trim("", None), "");
    assert_eq!(trim("hello", None), "hello");
    assert_eq!(trim("   ", None), "");
}

#[test]
fn test_trim_start() {
    assert_eq!(trim_start("  hello world  ", None), "hello world  ");
    assert_eq!(trim_start("__hello__", Some("_")), "hello__");
    assert_eq!(trim_start("-!-hello-!-", Some("-!")), "hello-!-");
    assert_eq!(trim_start("/path/to/file/", Some("/")), "path/to/file/");
    assert_eq!(trim_start("", None), "");
    assert_eq!(trim_start("hello", None), "hello");
}

#[test]
fn test_trim_end() {
    assert_eq!(trim_end("  hello world  ", None), "  hello world");
    assert_eq!(trim_end("__hello__", Some("_")), "__hello");
    assert_eq!(trim_end("-!-hello-!-", Some("-!")), "-!-hello");
    assert_eq!(trim_end("/path/to/file/", Some("/")), "/path/to/file");
    assert_eq!(trim_end("", None), "");
    assert_eq!(trim_end("hello", None), "hello");
}

#[test]
fn test_remove_prefix() {
    assert_eq!(remove_prefix("hello world", "hello "), "world");
    assert_eq!(remove_prefix("__hello__", "__"), "hello__");
    assert_eq!(remove_prefix("test", "no"), "test");
    assert_eq!(remove_prefix("", "prefix"), "");
    assert_eq!(remove_prefix("hello", ""), "hello");
    assert_eq!(remove_prefix("hello", "hello"), "");
}

#[test]
fn test_generate_merge_paths() {
    let branches = vec!["dev-xxx".to_string(), "dev".to_string(), "test".to_string()];
    let paths = generate_merge_paths(&branches);
    assert_eq!(paths, vec![
        vec!["dev-xxx".to_string(), "dev".to_string()],
        vec!["dev".to_string(), "test".to_string()]
    ]);
    
    let branches2 = vec!["feature".to_string(), "dev".to_string(), "test".to_string(), "prod".to_string()];
    let paths2 = generate_merge_paths(&branches2);
    assert_eq!(paths2, vec![
        vec!["feature".to_string(), "dev".to_string()],
        vec!["dev".to_string(), "test".to_string()],
        vec!["test".to_string(), "prod".to_string()]
    ]);
    
    // Test edge cases
    let empty_branches: Vec<String> = vec![];
    let empty_paths = generate_merge_paths(&empty_branches);
    assert_eq!(empty_paths, Vec::<Vec<String>>::new());
    
    let single_branch = vec!["main".to_string()];
    let single_paths = generate_merge_paths(&single_branch);
    assert_eq!(single_paths, Vec::<Vec<String>>::new());
}

#[test]
fn test_string_error() {
    let error = StringError::InvalidInput {
        message: "Test error".to_string(),
    };
    assert_eq!(error.to_string(), "Invalid input: Test error");
    
    let regex_error = StringError::RegexError {
        message: "Invalid regex".to_string(),
    };
    assert_eq!(regex_error.to_string(), "Regex error: Invalid regex");
}