//! Sliding Windows. Notes live in book/src/patterns/sliding_windows.md.

// ANCHOR: fixed_window
/// Fixed size k: add the entering element, subtract the leaving one. O(n)
pub fn max_sum_k(nums: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > nums.len() {
        return None;
    }
    let mut sum: i32 = nums[..k].iter().sum();
    let mut best = sum;
    for i in k..nums.len() {
        sum += nums[i] - nums[i - k];
        best = best.max(sum);
    }
    Some(best)
}
// ANCHOR_END: fixed_window

// ANCHOR: variable_window
/// 3. Longest Substring Without Repeating Characters.
/// Variable size: grow `r` every step; shrink `l` WHILE the window is invalid; record.
pub fn longest_unique_substring(s: &str) -> usize {
    let b = s.as_bytes();
    let mut counts = [0usize; 256]; // window contents (ASCII)
    let (mut l, mut best) = (0, 0);
    for r in 0..b.len() {
        counts[b[r] as usize] += 1; //          1) grow
        while counts[b[r] as usize] > 1 {
            counts[b[l] as usize] -= 1; //      2) shrink until valid
            l += 1;
        }
        best = best.max(r - l + 1); //          3) record (window is valid here)
    }
    best
}
// ANCHOR_END: variable_window

// ANCHOR: min_window
/// 209. Minimum Size Subarray Sum: the "shortest valid" variant. Record INSIDE the shrink loop.
pub fn min_subarray_len(target: i32, nums: &[i32]) -> Option<usize> {
    let (mut l, mut sum, mut best) = (0, 0, usize::MAX);
    for r in 0..nums.len() {
        sum += nums[r];
        while sum >= target {
            best = best.min(r - l + 1);
            sum -= nums[l];
            l += 1;
        }
    }
    (best != usize::MAX).then_some(best)
}
// ANCHOR_END: min_window

#[test]
fn test_sliding_window() {
    assert_eq!(max_sum_k(&[2, 1, 5, 1, 3, 2], 3), Some(9));
    assert_eq!(max_sum_k(&[1], 2), None);
    assert_eq!(longest_unique_substring("abcabcbb"), 3);
    assert_eq!(longest_unique_substring("bbbbb"), 1);
    assert_eq!(longest_unique_substring(""), 0);
    assert_eq!(min_subarray_len(7, &[2, 3, 1, 2, 4, 3]), Some(2));
    assert_eq!(min_subarray_len(100, &[1, 2]), None);
}
