//! Environment detection utilities
//!
//! This module provides utilities for detecting the current runtime environment
//! and platform-specific features.

use std::collections::HashMap;
use std::env;

/// Environment information structure
#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
    pub os: String,
    pub arch: String,
    pub family: String,
    pub exe_suffix: String,
    pub dll_suffix: String,
    pub dll_prefix: String,
    pub is_debug: bool,
    pub is_release: bool,
}

/// Get comprehensive environment information
pub fn get_environment_info() -> EnvironmentInfo {
    EnvironmentInfo {
        os: env::consts::OS.to_string(),
        arch: env::consts::ARCH.to_string(),
        family: env::consts::FAMILY.to_string(),
        exe_suffix: env::consts::EXE_SUFFIX.to_string(),
        dll_suffix: env::consts::DLL_SUFFIX.to_string(),
        dll_prefix: env::consts::DLL_PREFIX.to_string(),
        is_debug: cfg!(debug_assertions),
        is_release: !cfg!(debug_assertions),
    }
}

/// Check if running on Windows
pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

/// Check if running on macOS
pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

/// Check if running on Linux
pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

/// Check if running on Unix-like system
pub fn is_unix() -> bool {
    cfg!(target_family = "unix")
}

/// Check if running in debug mode
pub fn is_debug() -> bool {
    cfg!(debug_assertions)
}

/// Check if running in release mode
pub fn is_release() -> bool {
    !cfg!(debug_assertions)
}

/// Check if running on 64-bit architecture
pub fn is_64bit() -> bool {
    cfg!(target_pointer_width = "64")
}

/// Check if running on 32-bit architecture
pub fn is_32bit() -> bool {
    cfg!(target_pointer_width = "32")
}

/// Get current working directory
pub fn get_current_dir() -> Result<String, std::io::Error> {
    env::current_dir().map(|path| path.to_string_lossy().to_string())
}

/// Get environment variable
pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// Get environment variable with default value
pub fn get_env_var_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Get all environment variables
pub fn get_all_env_vars() -> HashMap<String, String> {
    env::vars().collect()
}

/// Check if environment variable exists
pub fn has_env_var(key: &str) -> bool {
    env::var(key).is_ok()
}

/// Get the number of CPU cores
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}

/// Get the number of physical CPU cores
pub fn get_physical_cpu_count() -> usize {
    num_cpus::get_physical()
}

/// Check if running in CI environment
pub fn is_ci() -> bool {
    has_env_var("CI")
        || has_env_var("CONTINUOUS_INTEGRATION")
        || has_env_var("GITHUB_ACTIONS")
        || has_env_var("GITLAB_CI")
        || has_env_var("TRAVIS")
        || has_env_var("CIRCLECI")
}

/// Get home directory path
pub fn get_home_dir() -> Option<String> {
    dirs::home_dir().map(|path| path.to_string_lossy().to_string())
}

/// Get config directory path
pub fn get_config_dir() -> Option<String> {
    dirs::config_dir().map(|path| path.to_string_lossy().to_string())
}

/// Get cache directory path
pub fn get_cache_dir() -> Option<String> {
    dirs::cache_dir().map(|path| path.to_string_lossy().to_string())
}

/// Get data directory path
pub fn get_data_dir() -> Option<String> {
    dirs::data_dir().map(|path| path.to_string_lossy().to_string())
}

/// Get temporary directory path
pub fn get_temp_dir() -> String {
    env::temp_dir().to_string_lossy().to_string()
}

/// Execute code only on specific operating system
pub fn run_on_os<T, F>(os: &str, func: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    if env::consts::OS == os {
        Some(func())
    } else {
        None
    }
}

/// Execute code only on Windows
pub fn run_on_windows<T, F>(func: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    if is_windows() { Some(func()) } else { None }
}

/// Execute code only on Unix-like systems
pub fn run_on_unix<T, F>(func: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    if is_unix() { Some(func()) } else { None }
}
