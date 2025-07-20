//! Integration tests for JavaScript-like array methods

use mudssky_utils::array::*;

#[test]
fn test_find_index() {
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(find_index(&vec, |&x| x > 3), Some(3));
    assert_eq!(find_index(&vec, |&x| x > 10), None);
    assert_eq!(find_index(&vec, |&x| x == 1), Some(0));

    let empty: Vec<i32> = vec![];
    assert_eq!(find_index(&empty, |&x| x > 0), None);
}

#[test]
fn test_find() {
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(find(&vec, |&x| x > 3), Some(&4));
    assert_eq!(find(&vec, |&x| x > 10), None);
    assert_eq!(find(&vec, |&x| x == 1), Some(&1));

    let empty: Vec<i32> = vec![];
    assert_eq!(find(&empty, |&x| x > 0), None);
}

#[test]
fn test_some() {
    let vec = vec![1, 2, 3, 4, 5];
    assert!(some(&vec, |&x| x > 3));
    assert!(!some(&vec, |&x| x > 10));
    assert!(some(&vec, |&x| x == 1));

    let empty: Vec<i32> = vec![];
    assert!(!some(&empty, |&x| x > 0));
}

#[test]
fn test_every() {
    let vec = vec![2, 4, 6, 8];
    assert!(every(&vec, |&x| x % 2 == 0));
    assert!(!every(&vec, |&x| x > 5));

    let mixed = vec![1, 2, 3, 4];
    assert!(!every(&mixed, |&x| x % 2 == 0));

    let empty: Vec<i32> = vec![];
    assert!(every(&empty, |&x| x > 0)); // vacuous truth
}

#[test]
fn test_filter() {
    let vec = vec![1, 2, 3, 4, 5];
    let filtered = filter(&vec, |&x| x % 2 == 0);
    assert_eq!(filtered, vec![2, 4]);

    let all_odd = filter(&vec, |&x| x % 2 == 1);
    assert_eq!(all_odd, vec![1, 3, 5]);

    let none = filter(&vec, |&x| x > 10);
    assert_eq!(none, Vec::<i32>::new());
}

#[test]
fn test_map() {
    let vec = vec![1, 2, 3, 4, 5];
    let doubled = map(&vec, |&x| x * 2);
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

    let strings = map(&vec, |&x| x.to_string());
    assert_eq!(strings, vec!["1", "2", "3", "4", "5"]);

    let empty: Vec<i32> = vec![];
    let mapped = map(&empty, |&x| x * 2);
    assert_eq!(mapped, Vec::<i32>::new());
}

#[test]
fn test_reduce() {
    let vec = vec![1, 2, 3, 4, 5];
    let sum = reduce(&vec, 0, |acc, &x| acc + x);
    assert_eq!(sum, 15);

    let product = reduce(&vec, 1, |acc, &x| acc * x);
    assert_eq!(product, 120);

    let concatenated = reduce(&vec, String::new(), |acc, &x| acc + &x.to_string());
    assert_eq!(concatenated, "12345");

    let empty: Vec<i32> = vec![];
    let empty_sum = reduce(&empty, 0, |acc, &x| acc + x);
    assert_eq!(empty_sum, 0);
}

#[test]
fn test_includes() {
    let vec = vec![1, 2, 3, 4, 5];
    assert!(includes(&vec, &3));
    assert!(!includes(&vec, &10));
    assert!(includes(&vec, &1));
    assert!(includes(&vec, &5));

    let empty: Vec<i32> = vec![];
    assert!(!includes(&empty, &1));
}

#[test]
fn test_index_of() {
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(index_of(&vec, &3), Some(2));
    assert_eq!(index_of(&vec, &10), None);
    assert_eq!(index_of(&vec, &1), Some(0));
    assert_eq!(index_of(&vec, &5), Some(4));

    let duplicates = vec![1, 2, 2, 3];
    assert_eq!(index_of(&duplicates, &2), Some(1)); // first occurrence
}

#[test]
fn test_join() {
    let vec = vec!["hello", "world", "rust"];
    assert_eq!(join(&vec, ", "), "hello, world, rust");
    assert_eq!(join(&vec, "-"), "hello-world-rust");
    assert_eq!(join(&vec, ""), "helloworldrust");

    let numbers = vec![1, 2, 3, 4];
    assert_eq!(join(&numbers, ", "), "1, 2, 3, 4");

    let empty: Vec<String> = vec![];
    assert_eq!(join(&empty, ", "), "");

    let single = vec!["test"];
    assert_eq!(join(&single, ", "), "test");
}

#[test]
fn test_reverse() {
    let vec = vec![1, 2, 3, 4, 5];
    let reversed = reverse(&vec);
    assert_eq!(reversed, vec![5, 4, 3, 2, 1]);
    assert_eq!(vec, vec![1, 2, 3, 4, 5]); // original unchanged

    let empty: Vec<i32> = vec![];
    let empty_reversed = reverse(&empty);
    assert_eq!(empty_reversed, Vec::<i32>::new());

    let single = vec![42];
    let single_reversed = reverse(&single);
    assert_eq!(single_reversed, vec![42]);
}

#[test]
fn test_slice() {
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(slice(&vec, 1, Some(4)), vec![2, 3, 4]);
    assert_eq!(slice(&vec, 2, None), vec![3, 4, 5]);
    assert_eq!(slice(&vec, 0, Some(2)), vec![1, 2]);
    assert_eq!(slice(&vec, 10, Some(20)), Vec::<i32>::new());
    assert_eq!(slice(&vec, 2, Some(2)), Vec::<i32>::new());
    assert_eq!(slice(&vec, 3, Some(1)), Vec::<i32>::new()); // start > end
}

#[test]
fn test_concat() {
    let vec1 = [1, 2];
    let vec2 = [3, 4];
    let vec3 = [5, 6];
    let result = concat(&[&vec1[..], &vec2[..], &vec3[..]]);
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);

    let empty: Vec<i32> = vec![];
    let with_empty = concat(&[&vec1[..], &empty[..], &vec2[..]]);
    assert_eq!(with_empty, vec![1, 2, 3, 4]);

    let single_array = concat(&[&vec1[..]]);
    assert_eq!(single_array, vec![1, 2]);

    let no_arrays: &[&[i32]] = &[];
    let empty_result = concat(no_arrays);
    assert_eq!(empty_result, Vec::<i32>::new());
}

#[test]
fn test_flat() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flattened = flat(&nested);
    assert_eq!(flattened, vec![1, 2, 3, 4, 5]);

    let with_empty = vec![vec![1, 2], vec![], vec![3, 4]];
    let flattened_with_empty = flat(&with_empty);
    assert_eq!(flattened_with_empty, vec![1, 2, 3, 4]);

    let empty_nested: Vec<Vec<i32>> = vec![];
    let empty_flattened = flat(&empty_nested);
    assert_eq!(empty_flattened, Vec::<i32>::new());

    let all_empty = vec![vec![], vec![], vec![]];
    let all_empty_flattened: Vec<i32> = flat(&all_empty);
    assert_eq!(all_empty_flattened, Vec::<i32>::new());
}

#[test]
fn test_array_methods_integration() {
    // Test chaining-like operations
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Filter even numbers, double them, then sum
    let evens = filter(&numbers, |&x| x % 2 == 0);
    let doubled = map(&evens, |&x| x * 2);
    let sum = reduce(&doubled, 0, |acc, &x| acc + x);
    assert_eq!(sum, 60); // (2+4+6+8+10) * 2 = 30 * 2 = 60

    // Test with strings
    let words = vec!["hello", "world", "rust", "programming"];
    let long_words = filter(&words, |&word| word.len() > 4);
    let joined = join(&long_words, " ");
    assert_eq!(joined, "hello world programming");

    // Test nested operations
    let nested_arrays = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let flattened = flat(&nested_arrays);
    let sliced = slice(&flattened, 1, Some(4));
    assert_eq!(sliced, vec![2, 3, 4]);
}
