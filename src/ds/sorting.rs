//! Sorting and comparisons.

use std::cmp::{Ordering, Reverse};

#[test]
fn sort_basics() {
    // ANCHOR: sort_basics
    let mut v = vec![5, 2, 9, 1];
    v.sort(); //                                  [1, 2, 5, 9]  stable
    v.sort_unstable(); //                         faster, no allocation, fine for primitives
    v.sort_by(|a, b| b.cmp(a)); //                [9, 5, 2, 1]  descending
    v.sort_by_key(|&x| Reverse(x)); //            also descending
    let mut words = vec!["bb", "a", "ccc", "dd"];
    words.sort_by_key(|w| w.len()); //            ["a", "bb", "dd", "ccc"] (stable: bb before dd)
    // sort a copy, keep the original:
    let original = vec![3, 1, 2];
    let mut sorted = original.clone();
    sorted.sort();
    // ANCHOR_END: sort_basics
    assert_eq!(v, [9, 5, 2, 1]);
    assert_eq!(words, ["a", "bb", "dd", "ccc"]);
    assert_eq!((original, sorted), (vec![3, 1, 2], vec![1, 2, 3]));
}

#[test]
fn multi_key() {
    // ANCHOR: multi_key
    // Sort by score DESC, then name ASC.
    let mut people = vec![("bob", 90), ("amy", 95), ("cat", 90)];
    people.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    // or with a tuple key:
    people.sort_by_key(|&(name, score)| (Reverse(score), name));
    // ANCHOR_END: multi_key
    assert_eq!(people, [("amy", 95), ("bob", 90), ("cat", 90)]);
}

#[test]
fn intervals() {
    // ANCHOR: intervals
    // Classic: sort intervals by start, then merge overlaps.
    let mut iv = vec![[8, 10], [1, 3], [2, 6], [15, 18]];
    iv.sort_unstable_by_key(|x| x[0]);
    let mut merged: Vec<[i32; 2]> = Vec::new();
    for [s, e] in iv {
        match merged.last_mut() {
            Some(last) if s <= last[1] => last[1] = last[1].max(e),
            _ => merged.push([s, e]),
        }
    }
    // ANCHOR_END: intervals
    assert_eq!(merged, [[1, 6], [8, 10], [15, 18]]);
}

#[test]
fn ordering_and_floats() {
    // ANCHOR: ordering_and_floats
    let o: Ordering = 3.cmp(&5); //                      Ordering::Less
    match o {
        Ordering::Less => {}
        Ordering::Equal => {}
        Ordering::Greater => {}
    }
    // f64 only has partial_cmp (NaN!). For sorting use total_cmp:
    let mut fs: Vec<f64> = vec![2.5, -1.0, 0.3];
    fs.sort_by(|a, b| a.total_cmp(b)); //                [-1.0, 0.3, 2.5]
    // k-th smallest in O(n) average, without fully sorting:
    let mut v = vec![7, 2, 9, 4, 1];
    let (_, kth, _) = v.select_nth_unstable(2); //       3rd smallest = 4
    // ANCHOR_END: ordering_and_floats
    assert_eq!(o, Ordering::Less);
    assert_eq!(fs, [-1.0, 0.3, 2.5]);
    assert_eq!(*kth, 4);
}
