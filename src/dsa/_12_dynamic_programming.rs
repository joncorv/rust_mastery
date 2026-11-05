//! Dynamic Programming. Notes live in book/src/patterns/dynamic_programming.md.

// ANCHOR: memo
/// 70. Climbing Stairs, top-down: recursion + a memo table. Vec<Option<_>> when the state
/// is a small integer; HashMap<state, _> when it isn't.
pub fn climb_stairs_memo(n: usize) -> u64 {
    fn go(n: usize, memo: &mut Vec<Option<u64>>) -> u64 {
        if n <= 1 {
            return 1;
        }
        if let Some(v) = memo[n] {
            return v;
        }
        let v = go(n - 1, memo) + go(n - 2, memo);
        memo[n] = Some(v);
        v
    }
    go(n, &mut vec![None; n + 1])
}
// ANCHOR_END: memo

// ANCHOR: tabulation
/// 70. Climbing Stairs, bottom-up: dp[i] only needs the last two values -> O(1) space.
pub fn climb_stairs(n: usize) -> u64 {
    let (mut prev, mut cur) = (1u64, 1u64); // dp[0], dp[1]
    for _ in 2..=n {
        (prev, cur) = (cur, prev + cur);
    }
    cur
}
// ANCHOR_END: tabulation

// ANCHOR: coin_change
/// 322. Coin Change: 1D table, dp[a] = fewest coins to make amount a.
pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; amount + 1]; // MAX = "impossible"
    dp[0] = 0;
    for a in 1..=amount {
        for &c in coins {
            if c <= a && dp[a - c] != usize::MAX {
                dp[a] = dp[a].min(dp[a - c] + 1); // check MAX first, or +1 overflows
            }
        }
    }
    (dp[amount] != usize::MAX).then_some(dp[amount])
}
// ANCHOR_END: coin_change

// ANCHOR: lcs
/// 1143. Longest Common Subsequence: 2D table with an extra row/col of zeros so
/// dp[i][j] (first i chars of a, first j of b) never indexes -1.
pub fn lcs(a: &str, b: &str) -> usize {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            dp[i][j] = if a[i - 1] == b[j - 1] { dp[i - 1][j - 1] + 1 } else { dp[i - 1][j].max(dp[i][j - 1]) };
        }
    }
    dp[a.len()][b.len()]
}
// ANCHOR_END: lcs

#[test]
fn test_dp() {
    for n in 0..20 {
        assert_eq!(climb_stairs(n), climb_stairs_memo(n));
    }
    assert_eq!(climb_stairs(5), 8);
    assert_eq!(coin_change(&[1, 2, 5], 11), Some(3));
    assert_eq!(coin_change(&[2], 3), None);
    assert_eq!(coin_change(&[1], 0), Some(0));
    assert_eq!(lcs("abcde", "ace"), 3);
    assert_eq!(lcs("abc", "def"), 0);
}
