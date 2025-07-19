//! Async utilities module
//!
//! This module provides asynchronous utility functions.

use std::time::Duration;
use tokio::time::sleep;

/// Creates an asynchronous delay function
///
/// # Arguments
///
/// * `ms` - Delay time in milliseconds
///
/// # Returns
///
/// A future that resolves after the specified time
///
/// # Examples
///
/// ```
/// use mudssky_utils::async_utils::sleep_async;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     // Delay for 1 second
///     sleep_async(1000).await;
///     println!("Executed after 1 second");
/// }
/// ```
pub async fn sleep_async(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}
