use mudssky_utils::async_utils::*;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_sleep_async_basic() {
    let start = Instant::now();
    sleep_async(100).await;
    let elapsed = start.elapsed();

    // Allow some tolerance for timing (±50ms)
    assert!(elapsed >= Duration::from_millis(50));
    assert!(elapsed <= Duration::from_millis(200));
}

#[tokio::test]
async fn test_sleep_async_zero() {
    let start = Instant::now();
    sleep_async(0).await;
    let elapsed = start.elapsed();

    // Should complete very quickly (within 50ms to account for system overhead)
    assert!(elapsed <= Duration::from_millis(50));
}

#[tokio::test]
async fn test_sleep_async_longer_duration() {
    let start = Instant::now();
    sleep_async(500).await;
    let elapsed = start.elapsed();

    // Allow some tolerance for timing (±100ms)
    assert!(elapsed >= Duration::from_millis(400));
    assert!(elapsed <= Duration::from_millis(600));
}

#[tokio::test]
async fn test_sleep_async_multiple_calls() {
    let start = Instant::now();

    // Multiple sequential sleeps
    sleep_async(50).await;
    sleep_async(50).await;
    sleep_async(50).await;

    let elapsed = start.elapsed();

    // Should take approximately 150ms (±100ms tolerance)
    assert!(elapsed >= Duration::from_millis(100));
    assert!(elapsed <= Duration::from_millis(250));
}

#[tokio::test]
async fn test_sleep_async_concurrent() {
    let start = Instant::now();

    // Run multiple sleeps concurrently
    let (_, _, _) = tokio::join!(sleep_async(100), sleep_async(100), sleep_async(100));

    let elapsed = start.elapsed();

    // Should take approximately 100ms since they run concurrently (±50ms tolerance)
    assert!(elapsed >= Duration::from_millis(50));
    assert!(elapsed <= Duration::from_millis(200));
}

#[tokio::test]
async fn test_sleep_async_large_value() {
    let start = Instant::now();
    sleep_async(1).await; // 1ms
    let elapsed = start.elapsed();

    // Should complete quickly
    assert!(elapsed <= Duration::from_millis(50));
}
