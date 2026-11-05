//! HashMap<K, V>: average O(1) insert, lookup and remove. No ordering.

use std::collections::HashMap;

#[test]
fn basics() {
    // ANCHOR: basics
    let mut map: HashMap<&str, i32> = HashMap::new();
    let old = map.insert("doug", 1); //       None (no previous value)
    let old2 = map.insert("doug", 2); //      Some(1), the value it replaced
    let v = map.get("doug").copied(); //      Some(2)  (.copied() drops the borrow)
    let has = map.contains_key("tim"); //     false
    if let Some(x) = map.get_mut("doug") {
        *x += 10; //                          12
    }
    let gone = map.remove("doug"); //         Some(12)
    let n = map.len();
    // ANCHOR_END: basics
    assert_eq!((old, old2, v, has, gone, n), (None, Some(1), Some(2), false, Some(12), 0));
}

#[test]
fn entry() {
    // ANCHOR: entry
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in "hello".chars() {
        *counts.entry(c).or_insert(0) += 1; // THE frequency-count idiom
    }
    // or_default() works when V: Default (0, empty Vec, empty String, ...)
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    for w in ["hi", "yo", "hey"] {
        groups.entry(w.len()).or_default().push(w);
    }
    // update-if-present, else insert
    counts.entry('z').and_modify(|n| *n += 1).or_insert(1);
    // ANCHOR_END: entry
    assert_eq!(counts[&'l'], 2);
    assert_eq!(counts[&'z'], 1);
    assert_eq!(groups[&2], ["hi", "yo"]);
}

#[test]
fn build_and_iterate() {
    // ANCHOR: build_and_iterate
    // collect from an iterator of (K, V) tuples
    let nums = [4, 7, 9];
    let index_of: HashMap<i32, usize> = nums.iter().enumerate().map(|(i, &x)| (x, i)).collect();

    for (k, v) in &index_of { /* arbitrary order! */ }
    let mut keys: Vec<&i32> = index_of.keys().collect();
    keys.sort(); // sort when you need deterministic output
    let total: usize = index_of.values().sum();
    // the entry with the highest value
    let best = index_of.iter().max_by_key(|&(_, &v)| v).map(|(&k, _)| k);
    // ANCHOR_END: build_and_iterate
    assert_eq!(keys, [&4, &7, &9]);
    assert_eq!((total, best), (3, Some(9)));
}

#[test]
fn indexing_gotcha() {
    // ANCHOR: indexing_gotcha
    let mut map = HashMap::from([("a", 1)]);
    let a = map["a"]; //            fine for reads, PANICS if the key is missing
    // map["b"] = 2;               // ERROR: IndexMut isn't implemented, use insert/entry
    let b = map.get("b").copied().unwrap_or(0); // safe read with default
    map.retain(|_, v| *v > 0); //   keep only entries that match
    // ANCHOR_END: indexing_gotcha
    assert_eq!((a, b), (1, 0));
}

#[test]
fn two_sum() {
    // ANCHOR: two_sum
    fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut seen: HashMap<i32, usize> = HashMap::new(); // value -> index
        for (i, &x) in nums.iter().enumerate() {
            if let Some(&j) = seen.get(&(target - x)) {
                return Some((j, i));
            }
            seen.insert(x, i);
        }
        None
    }
    // ANCHOR_END: two_sum
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(two_sum(&[1, 2], 9), None);
}
