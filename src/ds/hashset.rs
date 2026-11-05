//! HashSet<T>: average O(1) insert, contains and remove. No duplicates, no ordering.

use std::collections::HashSet;

#[test]
fn basics() {
    // ANCHOR: basics
    let mut set: HashSet<i32> = HashSet::new();
    let added = set.insert(5); //       true (newly added)
    let again = set.insert(5); //       false (already present), handy for "seen before?"
    let has = set.contains(&5); //      true
    let removed = set.remove(&5); //    true
    let from_vec: HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect(); // {1, 2, 3}
    let has_dups = from_vec.len() != 4; // true, the "contains duplicate" one-liner
    // ANCHOR_END: basics
    assert_eq!((added, again, has, removed, has_dups), (true, false, true, true, true));
}

#[test]
fn set_ops() {
    // ANCHOR: set_ops
    let a: HashSet<i32> = [1, 2, 3].into();
    let b: HashSet<i32> = [2, 3, 4].into();
    let both: HashSet<i32> = a.intersection(&b).copied().collect(); //         {2, 3}
    let either: HashSet<i32> = a.union(&b).copied().collect(); //              {1, 2, 3, 4}
    let only_a: HashSet<i32> = a.difference(&b).copied().collect(); //         {1}
    let one_side: HashSet<i32> = a.symmetric_difference(&b).copied().collect(); // {1, 4}
    let sub = a.is_subset(&either); //                                         true
    let disjoint = a.is_disjoint(&b); //                                       false
    // operator forms (on references) build new sets:
    let both2 = &a & &b;
    // ANCHOR_END: set_ops
    assert_eq!(both, [2, 3].into());
    assert_eq!(either, [1, 2, 3, 4].into());
    assert_eq!(only_a, [1].into());
    assert_eq!(one_side, [1, 4].into());
    assert!(sub && !disjoint);
    assert_eq!(both2, both);
}

#[test]
fn filter_out() {
    // ANCHOR: filter_out
    let mut set: HashSet<i32> = (0..8).collect();
    set.retain(|&x| x != 0); // keep matches in place
    // move matches OUT into a new collection (Rust 1.88+)
    let evens: HashSet<i32> = set.extract_if(|v| v % 2 == 0).collect();
    // ANCHOR_END: filter_out
    assert_eq!(evens, [2, 4, 6].into());
    assert_eq!(set, [1, 3, 5, 7].into());
}

#[test]
fn hashable_keys() {
    // ANCHOR: hashable_keys
    // tuples, arrays, Vec, String and your own #[derive(Hash, Eq, PartialEq)] types work as keys
    let mut visited: HashSet<(usize, usize)> = HashSet::new(); // grid coordinates
    visited.insert((0, 1));
    let mut anagram_keys: HashSet<[u8; 26]> = HashSet::new(); // letter-count signature
    let mut sig = [0u8; 26];
    for b in "cab".bytes() {
        sig[(b - b'a') as usize] += 1;
    }
    anagram_keys.insert(sig);
    // f32/f64 are NOT Hash (NaN != NaN). Use integers, or f64::to_bits() if you must.
    // ANCHOR_END: hashable_keys
    assert!(visited.contains(&(0, 1)));
    assert_eq!(anagram_keys.len(), 1);
}
