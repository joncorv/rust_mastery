//! Sorting. Notes live in book/src/patterns/sorting.md.

use std::collections::HashMap;

// ANCHOR: merge_intervals
/// 56. Merge Intervals: sort by start, then extend or push.
pub fn merge_intervals(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    intervals.sort_unstable_by_key(|iv| iv[0]);
    let mut merged: Vec<[i32; 2]> = Vec::new();
    for [start, end] in intervals {
        match merged.last_mut() {
            Some(last) if start <= last[1] => last[1] = last[1].max(end),
            _ => merged.push([start, end]),
        }
    }
    merged
}
// ANCHOR_END: merge_intervals

// ANCHOR: sort_by_frequency
/// 451. Sort Characters By Frequency: count, then sort with a multi-key comparator.
pub fn sort_by_frequency(s: &str) -> String {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let mut pairs: Vec<(char, usize)> = counts.into_iter().collect();
    // count DESC, then char ASC as a tie-breaker so the output is deterministic
    pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.into_iter().map(|(c, n)| c.to_string().repeat(n)).collect()
}
// ANCHOR_END: sort_by_frequency

// ANCHOR: counting_sort
/// Counting sort: O(n + k) when values live in a small known range 0..k.
pub fn counting_sort(nums: &[usize], k: usize) -> Vec<usize> {
    let mut counts = vec![0; k];
    for &x in nums {
        counts[x] += 1;
    }
    counts.iter().enumerate().flat_map(|(value, &n)| std::iter::repeat_n(value, n)).collect()
}
// ANCHOR_END: counting_sort

#[test]
fn test_sorting() {
    assert_eq!(merge_intervals(vec![[8, 10], [1, 3], [2, 6], [15, 18]]), [[1, 6], [8, 10], [15, 18]]);
    assert_eq!(merge_intervals(vec![[1, 4], [4, 5]]), [[1, 5]]);
    assert_eq!(merge_intervals(vec![]), Vec::<[i32; 2]>::new());
    assert_eq!(sort_by_frequency("tree"), "eert");
    assert_eq!(counting_sort(&[2, 0, 2, 1, 1, 0], 3), [0, 0, 1, 1, 2, 2]);
}
