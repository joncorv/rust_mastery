//! Greedy. Notes live in book/src/patterns/greedy.md.

// ANCHOR: max_profit
/// 121. Best Time to Buy and Sell Stock: track the best "buy" seen so far.
pub fn max_profit(prices: &[i32]) -> i32 {
    let (mut min_price, mut best) = (i32::MAX, 0);
    for &p in prices {
        min_price = min_price.min(p);
        best = best.max(p - min_price);
    }
    best
}
// ANCHOR_END: max_profit

// ANCHOR: can_jump
/// 55. Jump Game: keep the farthest index reachable so far.
pub fn can_jump(nums: &[usize]) -> bool {
    let mut reach = 0;
    for (i, &n) in nums.iter().enumerate() {
        if i > reach {
            return false;
        }
        reach = reach.max(i + n);
    }
    true
}
// ANCHOR_END: can_jump

// ANCHOR: interval_scheduling
/// Max non-overlapping intervals (435 is the complement): sort by END, take whatever fits.
pub fn max_non_overlapping(mut intervals: Vec<(i32, i32)>) -> usize {
    intervals.sort_unstable_by_key(|&(_, end)| end);
    let (mut count, mut last_end) = (0, i32::MIN);
    for (start, end) in intervals {
        if start >= last_end {
            count += 1;
            last_end = end;
        }
    }
    count
}
// ANCHOR_END: interval_scheduling

#[test]
fn test_greedy() {
    assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(&[7, 6, 4]), 0);
    assert!(can_jump(&[2, 3, 1, 1, 4]));
    assert!(!can_jump(&[3, 2, 1, 0, 4]));
    assert_eq!(max_non_overlapping(vec![(1, 2), (2, 3), (3, 4), (1, 3)]), 3);
}
