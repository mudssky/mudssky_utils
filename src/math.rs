//! Math utilities for random number generation and array operations
//!
//! This module provides mathematical utility functions including random number generation
//! and array manipulation functions.

use rand::{Rng, rng};
use thiserror::Error;

/// Errors that can occur during math operations
#[derive(Error, Debug, PartialEq)]
pub enum MathError {
    #[error("Invalid argument: {message}")]
    InvalidArgument { message: String },
}

/// Generate a random integer in the range [start, end)
///
/// # Arguments
///
/// * `start` - Starting value (inclusive)
/// * `end` - Ending value (exclusive)
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::math::random_int;
///
/// let num = random_int(0, 100).unwrap();
/// assert!(num >= 0 && num < 100);
///
/// let num = random_int(10, 20).unwrap();
/// assert!(num >= 10 && num < 20);
/// ```
///
/// # Errors
///
/// Returns `MathError::InvalidArgument` if start >= end
pub fn random_int(start: i32, end: i32) -> Result<i32, MathError> {
    if start >= end {
        return Err(MathError::InvalidArgument {
            message: "start should be less than end".to_string(),
        });
    }

    let mut rng = rng();
    Ok(rng.random_range(start..end))
}

/// Generate a random integer in the range [0, max)
///
/// # Arguments
///
/// * `max` - Maximum value (exclusive)
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::math::random_int_max;
///
/// let num = random_int_max(100).unwrap();
/// assert!(num >= 0 && num < 100);
/// ```
///
/// # Errors
///
/// Returns `MathError::InvalidArgument` if max <= 0
pub fn random_int_max(max: i32) -> Result<i32, MathError> {
    if max <= 0 {
        return Err(MathError::InvalidArgument {
            message: "max should be greater than 0".to_string(),
        });
    }

    random_int(0, max)
}

/// Get a random item from an array
///
/// # Arguments
///
/// * `arr` - The array to select from
///
/// # Examples
///
/// ```rust
/// use mudssky_utils::math::get_random_item_from_array;
///
/// let arr = vec![1, 2, 3, 4, 5];
/// let item = get_random_item_from_array(&arr).unwrap();
/// assert!(arr.contains(&item));
/// ```
///
/// # Errors
///
/// Returns `MathError::InvalidArgument` if the array is empty
pub fn get_random_item_from_array<T: Clone>(arr: &[T]) -> Result<T, MathError> {
    if arr.is_empty() {
        return Err(MathError::InvalidArgument {
            message: "array should not be empty".to_string(),
        });
    }

    let mut rng = rng();
    let index = rng.random_range(0..arr.len());
    Ok(arr[index].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_int() {
        let result = random_int(0, 10);
        assert!(result.is_ok());
        let num = result.unwrap();
        assert!(num >= 0 && num < 10);
    }

    #[test]
    fn test_random_int_invalid_range() {
        let result = random_int(10, 5);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MathError::InvalidArgument {
                message: "start should be less than end".to_string()
            }
        );
    }

    #[test]
    fn test_random_int_max() {
        let result = random_int_max(100);
        assert!(result.is_ok());
        let num = result.unwrap();
        assert!(num >= 0 && num < 100);
    }

    #[test]
    fn test_random_int_max_invalid() {
        let result = random_int_max(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MathError::InvalidArgument {
                message: "max should be greater than 0".to_string()
            }
        );
    }

    #[test]
    fn test_get_random_item_from_array() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = get_random_item_from_array(&arr);
        assert!(result.is_ok());
        let item = result.unwrap();
        assert!(arr.contains(&item));
    }

    #[test]
    fn test_get_random_item_from_empty_array() {
        let arr: Vec<i32> = vec![];
        let result = get_random_item_from_array(&arr);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MathError::InvalidArgument {
                message: "array should not be empty".to_string()
            }
        );
    }
}
