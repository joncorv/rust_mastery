//! Prefix Sums. Notes live in book/src/patterns/prefix_sums.md.

use std::collections::HashMap;

// ANCHOR: prefix
/// prefix[i] = sum of nums[..i]. Length n + 1 with a leading 0, so there are no edge cases.
pub fn prefix_sums(nums: &[i64]) -> Vec<i64> {
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    prefix
}

/// Sum of nums[l..=r] in O(1).
pub fn range_sum(prefix: &[i64], l: usize, r: usize) -> i64 {
    prefix[r + 1] - prefix[l]
}
// ANCHOR_END: prefix

// ANCHOR: subarray_sum_k
/// 560. Subarray Sum Equals K: prefix sums + "have I seen (sum - k) before?"
/// Works with negatives, where a sliding window doesn't.
pub fn subarray_sum_k(nums: &[i32], k: i32) -> usize {
    let mut seen: HashMap<i32, usize> = HashMap::from([(0, 1)]); // empty prefix
    let (mut sum, mut count) = (0, 0);
    for &x in nums {
        sum += x;
        count += seen.get(&(sum - k)).copied().unwrap_or(0);
        *seen.entry(sum).or_insert(0) += 1;
    }
    count
}
// ANCHOR_END: subarray_sum_k

// ANCHOR: product_except_self
/// 238. Product of Array Except Self: prefix and suffix PRODUCTS, no division.
pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut out = vec![1; n];
    for i in 1..n {
        out[i] = out[i - 1] * nums[i - 1]; // product of everything left of i
    }
    let mut right = 1;
    for i in (0..n).rev() {
        out[i] *= right; //                   times everything right of i
        right *= nums[i];
    }
    out
}
// ANCHOR_END: product_except_self

#[test]
fn test_prefix_sums() {
    let p = prefix_sums(&[3, 1, 4, 1, 5]);
    assert_eq!(p, [0, 3, 4, 8, 9, 14]);
    assert_eq!(range_sum(&p, 1, 3), 6);
    assert_eq!(range_sum(&p, 0, 4), 14);
    assert_eq!(subarray_sum_k(&[1, 1, 1], 2), 2);
    assert_eq!(subarray_sum_k(&[1, -1, 0], 0), 3);
    assert_eq!(product_except_self(&[1, 2, 3, 4]), [24, 12, 8, 6]);
}
