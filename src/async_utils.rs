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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_sleep_async() {
        let start = Instant::now();
        sleep_async(100).await;
        let elapsed = start.elapsed();

        // Allow some tolerance for timing
        assert!(elapsed >= Duration::from_millis(90));
        assert!(elapsed <= Duration::from_millis(150));
    }

    #[tokio::test]
    async fn test_sleep_async_zero() {
        let start = Instant::now();
        sleep_async(0).await;
        let elapsed = start.elapsed();

        // Should complete very quickly (within 50ms to account for system overhead)
        assert!(elapsed <= Duration::from_millis(50));
    }
}
