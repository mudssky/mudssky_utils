//! Integration tests for environment utilities

use mudssky_utils::env::*;

#[test]
fn test_environment_info() {
    let info = get_environment_info();

    // These should always have values
    assert!(!info.os.is_empty());
    assert!(!info.arch.is_empty());
    assert!(!info.family.is_empty());

    // Check that we get expected values
    assert!(matches!(info.os.as_str(), "windows" | "linux" | "macos"));
    assert!(matches!(
        info.arch.as_str(),
        "x86" | "x86_64" | "aarch64" | "arm"
    ));
    assert!(matches!(info.family.as_str(), "windows" | "unix"));
}

#[test]
fn test_os_detection() {
    assert!(is_windows() || is_linux() || is_macos());

    // Only one should be true
    let count = [is_windows(), is_linux(), is_macos()].iter().filter(|&&x| x).count();
    assert_eq!(count, 1);
}

#[test]
fn test_architecture() {
    let info = get_environment_info();
    let arch = info.arch;
    assert!(matches!(
        arch.as_str(),
        "x86" | "x86_64" | "aarch64" | "arm"
    ));
}

#[test]
fn test_build_mode() {
    // This will be true in debug builds, false in release builds
    let is_debug = is_debug();
    // Test that the function returns a boolean value
    let _ = is_debug; // Just ensure the function can be called
}

#[test]
fn test_current_dir() {
    let current = get_current_dir();
    assert!(current.is_ok());

    if let Ok(path) = current {
        let path_buf = std::path::Path::new(&path);
        assert!(path_buf.exists());
        assert!(path_buf.is_dir());
    }
}

#[test]
fn test_env_var() {
    // Test with a variable that should exist
    let path = get_env_var("PATH");
    assert!(path.is_some());
    assert!(!path.unwrap().is_empty());

    // Test with a variable that shouldn't exist
    let nonexistent = get_env_var("NONEXISTENT_VAR_12345");
    assert!(nonexistent.is_none());
}

#[test]
fn test_env_var_default() {
    // Test with existing variable
    let path = get_env_var_or_default("PATH", "default_value");
    assert_ne!(path, "default_value");

    // Test with non-existing variable
    let nonexistent = get_env_var_or_default("NONEXISTENT_VAR_12345", "default_value");
    assert_eq!(nonexistent, "default_value");
}

#[test]
fn test_cpu_count() {
    let count = get_cpu_count();
    assert!(count > 0);
    assert!(count <= 1024); // Reasonable upper bound
}

#[test]
fn test_temp_dir() {
    let temp = get_temp_dir();
    let temp_path = std::path::Path::new(&temp);
    assert!(temp_path.exists());
    assert!(temp_path.is_dir());
}

#[test]
fn test_run_on_os() {
    let mut windows_executed = false;
    let mut linux_executed = false;
    let mut macos_executed = false;

    run_on_windows(|| {
        windows_executed = true;
    });

    run_on_os("linux", || {
        linux_executed = true;
    });

    run_on_os("macos", || {
        macos_executed = true;
    });

    // Only one should have executed
    let executed_count = [windows_executed, linux_executed, macos_executed]
        .iter()
        .filter(|&&x| x)
        .count();
    assert_eq!(executed_count, 1);

    // Check that the right one executed
    if is_windows() {
        assert!(windows_executed);
    } else if is_linux() {
        assert!(linux_executed);
    } else if is_macos() {
        assert!(macos_executed);
    }
}

#[test]
fn test_conditional_execution() {
    let mut debug_executed = false;
    let mut release_executed = false;

    if is_debug() {
        debug_executed = true;
    }

    if is_release() {
        release_executed = true;
    }

    // Only one should have executed
    let executed_count = [debug_executed, release_executed].iter().filter(|&&x| x).count();
    assert_eq!(executed_count, 1);

    // Check that the right one executed
    if is_debug() {
        assert!(debug_executed);
    } else {
        assert!(release_executed);
    }
}
