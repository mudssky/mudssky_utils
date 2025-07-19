//! Function utilities module
//!
//! This module provides utilities for function manipulation including debouncing,
//! throttling, polling, and retry mechanisms.

use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Error types for function utilities
#[derive(Debug, Clone)]
pub enum FunctionError {
    /// Timeout error
    Timeout(String),
    /// Retry exhausted error
    RetryExhausted(String),
    /// Polling error
    PollingError(String),
    /// General error
    General(String),
}

impl std::fmt::Display for FunctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionError::Timeout(msg) => write!(f, "Timeout error: {msg}"),
            FunctionError::RetryExhausted(msg) => write!(f, "Retry exhausted: {msg}"),
            FunctionError::PollingError(msg) => write!(f, "Polling error: {msg}"),
            FunctionError::General(msg) => write!(f, "Function error: {msg}"),
        }
    }
}

impl std::error::Error for FunctionError {}

/// Debounce options
#[derive(Debug, Clone)]
pub struct DebounceOptions {
    /// Execute on leading edge
    pub leading: bool,
    /// Execute on trailing edge
    pub trailing: bool,
}

impl Default for DebounceOptions {
    fn default() -> Self {
        Self {
            leading: false,
            trailing: true,
        }
    }
}

/// Debounce controller
#[derive(Debug)]
pub struct Debouncer {
    last_call: Arc<Mutex<Option<Instant>>>,
    wait_duration: Duration,
    options: DebounceOptions,
    is_cancelled: Arc<AtomicBool>,
}

impl Debouncer {
    /// Create a new debouncer
    ///
    /// # Examples
    ///
    /// ```
    /// use mudssky_utils::function::{Debouncer, DebounceOptions};
    /// use std::time::Duration;
    ///
    /// let debouncer = Debouncer::new(Duration::from_millis(200), DebounceOptions::default());
    /// ```
    pub fn new(wait_duration: Duration, options: DebounceOptions) -> Self {
        Self {
            last_call: Arc::new(Mutex::new(None)),
            wait_duration,
            options,
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Execute a function with debouncing
    pub async fn execute<F, Fut, T>(&self, func: F) -> Result<T, FunctionError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        let now = Instant::now();

        {
            let mut last_call = self.last_call.lock().unwrap();
            *last_call = Some(now);
        }

        if self.options.leading {
            return Ok(func().await);
        }

        sleep(self.wait_duration).await;

        if self.is_cancelled.load(Ordering::Relaxed) {
            return Err(FunctionError::General(
                "Debouncer was cancelled".to_string(),
            ));
        }

        let should_execute = {
            let last_call = self.last_call.lock().unwrap();
            if let Some(_last) = *last_call {
                now.elapsed() >= self.wait_duration
            } else {
                false
            }
        };

        if should_execute && self.options.trailing {
            Ok(func().await)
        } else {
            Err(FunctionError::General(
                "Function execution was debounced".to_string(),
            ))
        }
    }

    /// Cancel the debouncer
    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
    }

    /// Check if debouncer is pending
    pub fn is_pending(&self) -> bool {
        let last_call = self.last_call.lock().unwrap();
        if let Some(last) = *last_call {
            last.elapsed() < self.wait_duration
        } else {
            false
        }
    }
}

/// Throttle options
#[derive(Debug, Clone)]
pub struct ThrottleOptions {
    /// Execute on leading edge
    pub leading: bool,
    /// Execute on trailing edge
    pub trailing: bool,
}

impl Default for ThrottleOptions {
    fn default() -> Self {
        Self {
            leading: false,
            trailing: true,
        }
    }
}

/// Throttle controller
#[derive(Debug)]
pub struct Throttler {
    last_execution: Arc<Mutex<Option<Instant>>>,
    wait_duration: Duration,
    options: ThrottleOptions,
    is_cancelled: Arc<AtomicBool>,
}

impl Throttler {
    /// Create a new throttler
    ///
    /// # Examples
    ///
    /// ```
    /// use mudssky_utils::function::{Throttler, ThrottleOptions};
    /// use std::time::Duration;
    ///
    /// let throttler = Throttler::new(Duration::from_millis(200), ThrottleOptions::default());
    /// ```
    pub fn new(wait_duration: Duration, options: ThrottleOptions) -> Self {
        Self {
            last_execution: Arc::new(Mutex::new(None)),
            wait_duration,
            options,
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Execute a function with throttling
    pub async fn execute<F, Fut, T>(&self, func: F) -> Result<T, FunctionError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        if self.is_cancelled.load(Ordering::Relaxed) {
            return Err(FunctionError::General(
                "Throttler was cancelled".to_string(),
            ));
        }

        let now = Instant::now();
        let should_execute = {
            let mut last_execution = self.last_execution.lock().unwrap();
            if let Some(last) = *last_execution {
                if now.duration_since(last) >= self.wait_duration {
                    *last_execution = Some(now);
                    true
                } else {
                    false
                }
            } else {
                *last_execution = Some(now);
                self.options.leading
            }
        };

        if should_execute {
            Ok(func().await)
        } else {
            Err(FunctionError::General(
                "Function execution was throttled".to_string(),
            ))
        }
    }

    /// Cancel the throttler
    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::Relaxed);
    }
}

/// Polling options
#[derive(Debug, Clone)]
pub struct PollingOptions {
    /// Polling interval
    pub interval: Duration,
    /// Maximum number of retries on error
    pub max_retries: usize,
    /// Whether to quit on error after max retries
    pub quit_on_error: bool,
    /// Whether to execute immediately
    pub immediate: bool,
    /// Maximum number of executions
    pub max_executions: usize,
}

impl Default for PollingOptions {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(5000),
            max_retries: 3,
            quit_on_error: true,
            immediate: false,
            max_executions: usize::MAX,
        }
    }
}

/// Polling status
#[derive(Debug, Clone)]
pub struct PollingStatus {
    /// Whether polling is active
    pub is_active: bool,
    /// Current retry count
    pub retry_count: usize,
    /// Current execution count
    pub execution_count: usize,
}

/// Polling controller
#[derive(Debug)]
pub struct Poller {
    options: PollingOptions,
    is_active: Arc<AtomicBool>,
    retry_count: Arc<Mutex<usize>>,
    execution_count: Arc<Mutex<usize>>,
}

impl Poller {
    /// Create a new poller
    ///
    /// # Examples
    ///
    /// ```
    /// use mudssky_utils::function::{Poller, PollingOptions};
    /// use std::time::Duration;
    ///
    /// let poller = Poller::new(PollingOptions {
    ///     interval: Duration::from_millis(2000),
    ///     ..Default::default()
    /// });
    /// ```
    pub fn new(options: PollingOptions) -> Self {
        Self {
            options,
            is_active: Arc::new(AtomicBool::new(false)),
            retry_count: Arc::new(Mutex::new(0)),
            execution_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Start polling with a task and stop condition
    pub async fn start<F, Fut, T, S>(&self, task: F, stop_condition: S) -> Result<T, FunctionError>
    where
        F: Fn() -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
        T: Clone + Send + Sync,
        S: Fn(&T) -> bool + Send + Sync,
    {
        self.is_active.store(true, Ordering::Relaxed);

        if self.options.immediate {
            match task().await {
                Ok(result) => {
                    if stop_condition(&result) {
                        return Ok(result);
                    }
                }
                Err(_) => {
                    let mut retry_count = self.retry_count.lock().unwrap();
                    *retry_count += 1;
                }
            }
        }

        while self.is_active.load(Ordering::Relaxed) {
            let execution_count = {
                let mut count = self.execution_count.lock().unwrap();
                *count += 1;
                *count
            };

            if execution_count > self.options.max_executions {
                break;
            }

            sleep(self.options.interval).await;

            if !self.is_active.load(Ordering::Relaxed) {
                break;
            }

            match task().await {
                Ok(result) => {
                    if stop_condition(&result) {
                        self.is_active.store(false, Ordering::Relaxed);
                        return Ok(result);
                    }
                }
                Err(_) => {
                    let retry_count = {
                        let mut count = self.retry_count.lock().unwrap();
                        *count += 1;
                        *count
                    };

                    if self.options.quit_on_error && retry_count >= self.options.max_retries {
                        self.is_active.store(false, Ordering::Relaxed);
                        return Err(FunctionError::PollingError(
                            "Max retries exceeded".to_string(),
                        ));
                    }
                }
            }
        }

        Err(FunctionError::PollingError("Polling stopped".to_string()))
    }

    /// Stop polling
    pub fn stop(&self) {
        self.is_active.store(false, Ordering::Relaxed);
    }

    /// Get polling status
    pub fn status(&self) -> PollingStatus {
        PollingStatus {
            is_active: self.is_active.load(Ordering::Relaxed),
            retry_count: *self.retry_count.lock().unwrap(),
            execution_count: *self.execution_count.lock().unwrap(),
        }
    }
}

/// Retry options
#[derive(Debug, Clone)]
pub struct RetryOptions {
    /// Maximum number of retries
    pub max_retries: usize,
    /// Delay between retries
    pub delay: Duration,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            max_retries: 3,
            delay: Duration::from_millis(0),
        }
    }
}

/// Execute a function with retry logic
///
/// # Examples
///
/// ```
/// use mudssky_utils::function::{with_retry, RetryOptions};
/// use std::time::Duration;
///
/// async fn example() {
///     let result = with_retry(
///         || async { Ok::<i32, Box<dyn std::error::Error + Send + Sync>>(42) },
///         RetryOptions {
///             max_retries: 3,
///             delay: Duration::from_millis(1000),
///         },
///     ).await;
/// }
/// ```
pub async fn with_retry<F, Fut, T>(func: F, options: RetryOptions) -> Result<T, FunctionError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
{
    let mut retry_count = 0;
    let mut last_error: Option<Box<dyn std::error::Error + Send + Sync>> = None;

    while retry_count <= options.max_retries {
        match func().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                last_error = Some(error);
                retry_count += 1;

                if retry_count <= options.max_retries && options.delay > Duration::from_millis(0) {
                    sleep(options.delay).await;
                }
            }
        }
    }

    Err(FunctionError::RetryExhausted(format!(
        "Function failed after {} retries. Last error: {}",
        options.max_retries,
        last_error.map(|e| e.to_string()).unwrap_or_else(|| "Unknown error".to_string())
    )))
}
