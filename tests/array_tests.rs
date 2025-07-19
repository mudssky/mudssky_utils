//! Integration tests for array module

use mudssky_utils::array::*;
use std::collections::HashMap;

#[test]
fn test_range() {
    // Test basic range
    assert_eq!(range(5, None, None).unwrap(), vec![0, 1, 2, 3, 4]);

    // Test range with start and end
    assert_eq!(range(1, Some(4), None).unwrap(), vec![1, 2, 3]);

    // Test range with step
    assert_eq!(range(0, Some(10), Some(2)).unwrap(), vec![0, 2, 4, 6, 8]);

    // Test negative step
    assert_eq!(range(10, Some(0), Some(-2)).unwrap(), vec![10, 8, 6, 4, 2]);

    // Test zero step error
    assert_eq!(range(0, Some(10), Some(0)), Err(ArrayError::ZeroStep));

    // Test empty range
    assert_eq!(range(5, Some(5), None).unwrap(), vec![]);
    assert_eq!(range(0, Some(0), None).unwrap(), vec![]);
}

#[test]
fn test_chunk() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];

    // Test normal chunking
    assert_eq!(chunk(&data, 3), vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    assert_eq!(
        chunk(&data, 2),
        vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7]]
    );

    // Test edge cases
    assert_eq!(
        chunk(&data, 1),
        vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
            vec![7]
        ]
    );
    assert_eq!(chunk(&data, 10), vec![vec![1, 2, 3, 4, 5, 6, 7]]);

    // Test empty array
    let empty: Vec<i32> = vec![];
    assert_eq!(chunk(&empty, 3), Vec::<Vec<i32>>::new());

    // Test zero size
    assert_eq!(chunk(&data, 0), Vec::<Vec<i32>>::new());
}

#[test]
fn test_first_and_last() {
    let data = vec![1, 2, 3, 4, 5];

    // Test first
    assert_eq!(first(&data, &0), &1);
    assert_eq!(first(&[], &42), &42);

    // Test last
    assert_eq!(last(&data, &0), &5);
    assert_eq!(last(&[], &42), &42);

    // Test single element
    let single = vec![99];
    assert_eq!(first(&single, &0), &99);
    assert_eq!(last(&single, &0), &99);
}

#[test]
fn test_count_by() {
    let words = vec!["apple", "banana", "apricot", "blueberry"];
    let counts = count_by(&words, |s| s.chars().next().unwrap());

    let mut expected = HashMap::new();
    expected.insert('a', 2);
    expected.insert('b', 2);
    assert_eq!(counts, expected);

    // Test with numbers
    let numbers = vec![1, 2, 3, 1, 2, 1];
    let num_counts = count_by(&numbers, |&x| x);

    let mut expected_nums = HashMap::new();
    expected_nums.insert(1, 3);
    expected_nums.insert(2, 2);
    expected_nums.insert(3, 1);
    assert_eq!(num_counts, expected_nums);

    // Test empty array
    let empty: Vec<i32> = vec![];
    let empty_counts = count_by(&empty, |&x| x);
    assert_eq!(empty_counts, HashMap::new());
}

#[test]
fn test_diff() {
    let a = vec![1, 2, 3, 4];
    let b = vec![2, 4, 6];
    assert_eq!(diff(&a, &b, |x| *x), vec![1, 3]);

    // Test with strings
    let words1 = vec!["apple", "banana", "cherry"];
    let words2 = vec!["banana", "date"];
    assert_eq!(diff(&words1, &words2, |s| *s), vec!["apple", "cherry"]);

    // Test empty arrays
    let empty: Vec<i32> = vec![];
    assert_eq!(diff(&a, &empty, |x| *x), vec![1, 2, 3, 4]);
    assert_eq!(diff(&empty, &b, |x| *x), Vec::<i32>::new());

    // Test identical arrays
    assert_eq!(diff(&a, &a, |x| *x), Vec::<i32>::new());
}

#[test]
fn test_fork() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (evens, odds) = fork(&numbers, |x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5]);

    // Test with strings
    let words = vec!["a", "bb", "ccc", "dd"];
    let (short, long) = fork(&words, |s| s.len() <= 2);
    assert_eq!(short, vec!["a", "bb", "dd"]);
    assert_eq!(long, vec!["ccc"]);

    // Test empty array
    let empty: Vec<i32> = vec![];
    let (empty1, empty2) = fork(&empty, |x| x % 2 == 0);
    assert_eq!(empty1, Vec::<i32>::new());
    assert_eq!(empty2, Vec::<i32>::new());
}

#[test]
fn test_max_and_min() {
    let numbers = vec![1, 3, 2, 5, 4];

    // Test max
    assert_eq!(max(&numbers, None::<fn(&i32) -> i32>), Some(&5));
    assert_eq!(max::<i32, i32, fn(&i32) -> i32>(&[], None), None);

    // Test min
    assert_eq!(min(&numbers, None::<fn(&i32) -> i32>), Some(&1));
    assert_eq!(min::<i32, i32, fn(&i32) -> i32>(&[], None), None);

    // Test with getter function
    let people = vec![("Alice", 25), ("Bob", 30), ("Charlie", 20)];
    assert_eq!(
        max(&people, Some(|p: &(&str, i32)| p.1)),
        Some(&("Bob", 30))
    );
    assert_eq!(
        min(&people, Some(|p: &(&str, i32)| p.1)),
        Some(&("Charlie", 20))
    );

    // Test single element
    let single = vec![42];
    assert_eq!(max(&single, None::<fn(&i32) -> i32>), Some(&42));
    assert_eq!(min(&single, None::<fn(&i32) -> i32>), Some(&42));
}

#[test]
fn test_sum() {
    // Test direct sum
    assert_eq!(sum_direct(&[1, 2, 3, 4]), 10);

    // Test with getter function
    let items = vec![("a", 1), ("b", 2), ("c", 3)];
    assert_eq!(sum(&items, |item| item.1), 6);

    // Test empty array
    let empty: Vec<(String, i32)> = vec![];
    assert_eq!(sum(&empty, |item| item.1), 0);

    // Test with floats
    let floats = vec![("x", 1.5), ("y", 2.5), ("z", 3.0)];
    assert_eq!(sum(&floats, |item| item.1), 7.0);

    // Test sum_direct with floats
    assert_eq!(sum_direct(&[1.5, 2.5, 3.0]), 7.0);
}

#[test]
fn test_unique() {
    // Test basic unique
    assert_eq!(unique(&[1, 2, 2, 3, 1], Some(|&x: &i32| x)), vec![1, 2, 3]);

    // Test with key function
    let people = vec![("Alice", 25), ("Bob", 30), ("Alice", 35)];
    let unique_names = unique(&people, Some(|p: &(&str, i32)| p.1));
    assert_eq!(
        unique_names,
        vec![("Alice", 25), ("Bob", 30), ("Alice", 35)]
    );

    // Test empty array
    let empty: Vec<i32> = vec![];
    assert_eq!(unique(&empty, Some(|&x: &i32| x)), Vec::<i32>::new());

    // Test already unique
    let already_unique = vec![1, 2, 3, 4];
    assert_eq!(
        unique(&already_unique, Some(|&x: &i32| x)),
        vec![1, 2, 3, 4]
    );

    // Test all same
    let all_same = vec![5, 5, 5, 5];
    assert_eq!(unique(&all_same, Some(|&x: &i32| x)), vec![5]);
}

#[test]
fn test_shuffle() {
    let original = vec![1, 2, 3, 4, 5];
    let shuffled = shuffle(&original);

    // Test that length is preserved
    assert_eq!(shuffled.len(), original.len());

    // Test that all elements are preserved (though order may change)
    let mut sorted_original = original.clone();
    sorted_original.sort();
    let mut sorted_shuffled = shuffled.clone();
    sorted_shuffled.sort();
    assert_eq!(sorted_original, sorted_shuffled);

    // Test empty array
    let empty: Vec<i32> = vec![];
    assert_eq!(shuffle(&empty), Vec::<i32>::new());

    // Test single element
    let single = vec![42];
    assert_eq!(shuffle(&single), vec![42]);
}

#[test]
fn test_array_error() {
    // Test error display
    let error = ArrayError::ZeroStep;
    assert_eq!(error.to_string(), "Step cannot be zero");

    let error2 = ArrayError::InvalidRange;
    assert_eq!(error2.to_string(), "Invalid range parameters");

    // Test error equality
    assert_eq!(ArrayError::ZeroStep, ArrayError::ZeroStep);
    assert_ne!(ArrayError::ZeroStep, ArrayError::InvalidRange);
}
