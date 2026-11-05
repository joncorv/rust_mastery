//! Binary Search. Notes live in book/src/patterns/binary_search.md; code is included by ANCHOR.

// ANCHOR: binary_search
/// My first solution: closed range with explicit edge checks.
pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None; // nums[0] and nums.len() - 1 would both panic
    }
    let (mut left, mut right) = (0, nums.len() - 1);

    // base cases
    if target < nums[0] || target > nums[right] {
        return None;
    } else if nums[0] == target {
        return Some(left);
    } else if nums[right] == target {
        return Some(right);
    }

    while left < right - 1 {
        let middle_index = left + (right - left) / 2;
        let middle_value = nums[middle_index];

        if middle_value == target {
            return Some(middle_index);
        } else if middle_value > target {
            right = middle_index;
        } else {
            left = middle_index
        }
    }
    None
}
// ANCHOR_END: binary_search

// ANCHOR: transition_point
/// The reusable template: find the FIRST index where `is_after` becomes true.
/// Everything before it is "before", everything from it on is "after".
/// Returns nums.len() if nothing is "after". Half-open range [lo, hi), so no usize
/// underflow and no off-by-one edge cases.
pub fn transition_point(nums: &[i32], is_after: impl Fn(i32) -> bool) -> usize {
    let (mut lo, mut hi) = (0, nums.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if is_after(nums[mid]) {
            hi = mid; // mid might be the answer: keep it
        } else {
            lo = mid + 1; // mid is definitely "before": drop it
        }
    }
    lo
}

/// Exact-match search, built on the template.
pub fn binary_search_v2(nums: &[i32], target: i32) -> Option<usize> {
    let i = transition_point(nums, |x| x >= target);
    (i < nums.len() && nums[i] == target).then_some(i)
}
// ANCHOR_END: transition_point

#[test]
pub fn test_binary_search() {
    for f in [binary_search, binary_search_v2] {
        assert_eq!(f(&[0, 1, 5, 8, 10, 13, 25, 78], 8), Some(3));
        assert_eq!(f(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10), Some(10));
        assert_eq!(f(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 20), None);
        assert_eq!(f(&[0, 2, 4], 3), None);
        assert_eq!(f(&[7], 7), Some(0));
        assert_eq!(f(&[], 1), None);
    }
    // the std equivalent of transition_point:
    assert_eq!([1, 3, 5, 7].partition_point(|&x| x < 4), transition_point(&[1, 3, 5, 7], |x| x >= 4));
}
