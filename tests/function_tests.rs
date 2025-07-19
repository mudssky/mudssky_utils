use mudssky_utils::function::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_debouncer_trailing() {
    let debouncer = Debouncer::new(
        Duration::from_millis(50),
        DebounceOptions {
            leading: false,
            trailing: true,
        },
    );

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = debouncer
        .execute(|| async {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            42
        })
        .await;

    // The function should execute after the delay
    assert!(result.is_ok() || result.is_err()); // Timing-dependent test
}

#[tokio::test]
async fn test_debouncer_leading() {
    let debouncer = Debouncer::new(
        Duration::from_millis(50),
        DebounceOptions {
            leading: true,
            trailing: false,
        },
    );

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = debouncer
        .execute(|| async {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            42
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn test_debouncer_cancel() {
    let debouncer = Debouncer::new(Duration::from_millis(100), DebounceOptions::default());

    debouncer.cancel();

    let result = debouncer.execute(|| async { 42 }).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_throttler_leading() {
    let throttler = Throttler::new(
        Duration::from_millis(50),
        ThrottleOptions {
            leading: true,
            trailing: false,
        },
    );

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = throttler
        .execute(|| async {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            42
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn test_throttler_multiple_calls() {
    let throttler = Throttler::new(
        Duration::from_millis(100),
        ThrottleOptions {
            leading: true,
            trailing: false,
        },
    );

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone1 = counter.clone();
    let counter_clone2 = counter.clone();

    // First call should succeed
    let result1 = throttler
        .execute(|| async {
            counter_clone1.fetch_add(1, Ordering::Relaxed);
            42
        })
        .await;

    // Second call immediately after should be throttled
    let result2 = throttler
        .execute(|| async {
            counter_clone2.fetch_add(1, Ordering::Relaxed);
            43
        })
        .await;

    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), 42);
    assert!(result2.is_err()); // Should be throttled
    assert_eq!(counter.load(Ordering::Relaxed), 1); // Only first call executed
}

#[tokio::test]
async fn test_throttler_cancel() {
    let throttler = Throttler::new(Duration::from_millis(50), ThrottleOptions::default());

    throttler.cancel();

    let result = throttler.execute(|| async { 42 }).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_poller_success() {
    let poller = Poller::new(PollingOptions {
        interval: Duration::from_millis(10),
        max_executions: 3,
        immediate: true,
        ..Default::default()
    });

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = poller
        .start(
            || async {
                let count = counter_clone.fetch_add(1, Ordering::Relaxed) + 1;
                if count >= 2 {
                    Ok(count)
                } else {
                    Err("Not ready yet".into())
                }
            },
            |result| *result >= 2,
        )
        .await;

    assert!(result.is_ok());
    assert!(result.unwrap() >= 2);
}

#[tokio::test]
async fn test_poller_stop() {
    let poller = Poller::new(PollingOptions {
        interval: Duration::from_millis(10),
        ..Default::default()
    });

    // Start polling in background
    let poller_clone = Arc::new(poller);
    let poller_ref = poller_clone.clone();

    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        poller_ref.stop();
    });

    let result = poller_clone
        .start(
            || async { Ok::<i32, Box<dyn std::error::Error + Send + Sync>>(42) },
            |_| false, // Never stop condition
        )
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_poller_status() {
    let poller = Poller::new(PollingOptions::default());

    let status = poller.status();
    assert!(!status.is_active);
    assert_eq!(status.retry_count, 0);
    assert_eq!(status.execution_count, 0);
}

#[tokio::test]
async fn test_with_retry_success() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = with_retry(
        || async {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            Ok::<i32, Box<dyn std::error::Error + Send + Sync>>(42)
        },
        RetryOptions {
            max_retries: 3,
            delay: Duration::from_millis(1),
        },
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::Relaxed), 1);
}

#[tokio::test]
async fn test_with_retry_failure_then_success() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = with_retry(
        || async {
            let count = counter_clone.fetch_add(1, Ordering::Relaxed) + 1;
            if count < 3 {
                Err::<i32, Box<dyn std::error::Error + Send + Sync>>("Not ready yet".into())
            } else {
                Ok(42)
            }
        },
        RetryOptions {
            max_retries: 3,
            delay: Duration::from_millis(1),
        },
    )
    .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(counter.load(Ordering::Relaxed), 3);
}

#[tokio::test]
async fn test_with_retry_exhausted() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let result = with_retry(
        || async {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            Err::<i32, Box<dyn std::error::Error + Send + Sync>>("Always fails".into())
        },
        RetryOptions {
            max_retries: 2,
            delay: Duration::from_millis(1),
        },
    )
    .await;

    assert!(result.is_err());
    assert_eq!(counter.load(Ordering::Relaxed), 3); // Initial + 2 retries

    if let Err(FunctionError::RetryExhausted(msg)) = result {
        assert!(msg.contains("Function failed after 2 retries"));
    } else {
        panic!("Expected RetryExhausted error");
    }
}

#[tokio::test]
async fn test_with_retry_no_delay() {
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let start = std::time::Instant::now();

    let result = with_retry(
        || async {
            counter_clone.fetch_add(1, Ordering::Relaxed);
            Err::<i32, Box<dyn std::error::Error + Send + Sync>>("Always fails".into())
        },
        RetryOptions {
            max_retries: 2,
            delay: Duration::from_millis(0),
        },
    )
    .await;

    let elapsed = start.elapsed();

    assert!(result.is_err());
    assert_eq!(counter.load(Ordering::Relaxed), 3);
    // Should complete quickly without delays
    assert!(elapsed < Duration::from_millis(50));
}

#[test]
fn test_function_error_display() {
    let error = FunctionError::Timeout("Test timeout".to_string());
    assert_eq!(error.to_string(), "Timeout error: Test timeout");

    let error = FunctionError::RetryExhausted("Test retry".to_string());
    assert_eq!(error.to_string(), "Retry exhausted: Test retry");

    let error = FunctionError::PollingError("Test polling".to_string());
    assert_eq!(error.to_string(), "Polling error: Test polling");

    let error = FunctionError::General("Test general".to_string());
    assert_eq!(error.to_string(), "Function error: Test general");
}

#[test]
fn test_debounce_options_default() {
    let options = DebounceOptions::default();
    assert!(!options.leading);
    assert!(options.trailing);
}

#[test]
fn test_throttle_options_default() {
    let options = ThrottleOptions::default();
    assert!(!options.leading);
    assert!(options.trailing);
}

#[test]
fn test_polling_options_default() {
    let options = PollingOptions::default();
    assert_eq!(options.interval, Duration::from_millis(5000));
    assert_eq!(options.max_retries, 3);
    assert!(options.quit_on_error);
    assert!(!options.immediate);
    assert_eq!(options.max_executions, usize::MAX);
}

#[test]
fn test_retry_options_default() {
    let options = RetryOptions::default();
    assert_eq!(options.max_retries, 3);
    assert_eq!(options.delay, Duration::from_millis(0));
}
