//! Recursion. Notes live in book/src/patterns/recursion.md.

// ANCHOR: fast_pow
/// 50. Pow(x, n): halve the problem each call -> O(log n) depth.
pub fn fast_pow(x: f64, n: i64) -> f64 {
    if n == 0 {
        return 1.0; // base case
    }
    if n < 0 {
        return 1.0 / fast_pow(x, -n);
    }
    let half = fast_pow(x, n / 2); // recursive case: trust it works for n/2
    if n % 2 == 0 { half * half } else { half * half * x }
}
// ANCHOR_END: fast_pow

// ANCHOR: slice_patterns
/// Slice patterns make "first, rest, last" recursion read like the math.
pub fn is_palindrome(s: &[u8]) -> bool {
    match s {
        [] | [_] => true,
        [first, middle @ .., last] => first == last && is_palindrome(middle),
    }
}

pub fn sum(nums: &[i32]) -> i32 {
    match nums {
        [] => 0,
        [head, tail @ ..] => head + sum(tail),
    }
}
// ANCHOR_END: slice_patterns

// ANCHOR: accumulator
/// Recursive data needs a recursive type: an enum with a Vec of itself.
pub enum Nested {
    Int(i32),
    List(Vec<Nested>),
}

/// The "helper with &mut accumulator" shape: no allocation per call, no returned Vecs to merge.
pub fn flatten(n: &Nested, out: &mut Vec<i32>) {
    match n {
        Nested::Int(x) => out.push(*x),
        Nested::List(items) => {
            for item in items {
                flatten(item, out);
            }
        }
    }
}
// ANCHOR_END: accumulator

#[test]
fn test_recursion() {
    assert_eq!(fast_pow(2.0, 10), 1024.0);
    assert_eq!(fast_pow(2.0, -2), 0.25);
    assert!(is_palindrome(b"racecar"));
    assert!(!is_palindrome(b"ab"));
    assert_eq!(sum(&[1, 2, 3]), 6);

    use Nested::*;
    let n = List(vec![Int(1), List(vec![Int(2), List(vec![Int(3)])]), Int(4)]);
    let mut out = vec![];
    flatten(&n, &mut out);
    assert_eq!(out, [1, 2, 3, 4]);
}
