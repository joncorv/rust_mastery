# Dynamic Programming

## Triggers

- **Count the ways**, **min / max cost**, or **is it possible** ...
- ... where choices at each step lead to **overlapping subproblems** (the naive recursion
  recomputes the same inputs).
- Sequences and strings: "longest increasing / common subsequence", "edit distance".
- Greedy feels right but you can find a counterexample.

## Keywords

| number of ways | minimum / maximum cost | longest subsequence | can you reach | knapsack |
|---|---|---|---|---|

## The big idea

1. **Define the state**: what does `dp[i]` (or `dp[i][j]`) mean, in one sentence?
2. **Recurrence**: how does `dp[i]` follow from smaller states?
3. **Base cases**: the smallest states.
4. **Order**: top-down (recursion + memo) or bottom-up (loops over a table).
5. **Answer**: which cell? Then optimize space if each row only needs the previous one.

Start **top-down**: it's the recursion you'd write anyway, plus a memo. Convert to
bottom-up if asked.

## Rust toolkit

- Memo: `Vec<Option<T>>` for integer states, `HashMap<(usize, usize), T>` for tuples.
- Tables: `vec![vec![0; m + 1]; n + 1]`. The **+1 row/col** of base cases avoids `i - 1` underflow.
- "Impossible" sentinel: `usize::MAX` / `i32::MAX`, but **check it before adding**.
- Rolling variables: `(prev, cur) = (cur, prev + cur);` (destructuring assignment).

## Template: top-down (memoized recursion)

```rust
{{#include ../../../src/dsa/_12_dynamic_programming.rs:memo}}
```

## Template: bottom-up, O(1) space

```rust
{{#include ../../../src/dsa/_12_dynamic_programming.rs:tabulation}}
```

## Template: 1D table (unbounded choices)

```rust
{{#include ../../../src/dsa/_12_dynamic_programming.rs:coin_change}}
```

## Template: 2D table (two sequences)

```rust
{{#include ../../../src/dsa/_12_dynamic_programming.rs:lcs}}
```

## Rust gotchas

- **`MAX + 1` overflows** (panic in debug, silent wrap in release). Guard with
  `!= usize::MAX` or use `checked_add`.
- **Memo as a `HashMap` passed to a closure** won't work (no recursive closures). Use a nested
  `fn` with `memo: &mut HashMap<..>`.
- **Answers that need "mod 1e9+7"**: apply `%` after every addition and use `i64`/`u64`.
- **Deep top-down recursion** (n ≈ 10⁵) can overflow the stack. Go bottom-up.

## Practice

- [ ] [Climbing Stairs (70)](https://leetcode.com/problems/climbing-stairs/)
- [ ] [House Robber (198)](https://leetcode.com/problems/house-robber/)
- [ ] [Coin Change (322)](https://leetcode.com/problems/coin-change/)
- [ ] [Longest Increasing Subsequence (300)](https://leetcode.com/problems/longest-increasing-subsequence/)
- [ ] [Unique Paths (62)](https://leetcode.com/problems/unique-paths/)
- [ ] [Longest Common Subsequence (1143)](https://leetcode.com/problems/longest-common-subsequence/)
- [ ] [Word Break (139)](https://leetcode.com/problems/word-break/)
- [ ] [Edit Distance (72)](https://leetcode.com/problems/edit-distance/)
