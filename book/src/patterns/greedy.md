# Greedy

## Triggers

- An optimization problem where **the locally best choice is always safe**: you never
  need to undo it.
- Often **after sorting**: by end time, by ratio, by deadline.
- Single pass with a running best: max profit, farthest reach, gas station.

## Keywords

| maximum number of | minimum number of | schedule | can reach | at each step |
|---|---|---|---|---|

## The big idea

Greedy is fast (usually O(n) or O(n log n)) but **easy to get wrong**. Before coding:

1. State the greedy rule in one sentence ("always take the interval that ends first").
2. Try to **break it** with a small counterexample. If you can, it's probably
   [DP](dynamic_programming.md).
3. Justify it out loud: *"any optimal solution can be swapped to include my choice without
   getting worse"* (the exchange argument).

## Rust toolkit

- `sort_unstable_by_key(|&(_, end)| end)` to order choices.
- Running state: `best = best.max(x)`, `min_so_far = min_so_far.min(x)`.
- `i32::MIN` / `i32::MAX` as starting sentinels.

## Template: running best

```rust
{{#include ../../../src/dsa/_13_greedy.rs:max_profit}}
```

## Template: farthest reach

```rust
{{#include ../../../src/dsa/_13_greedy.rs:can_jump}}
```

## Template: sort, then take what fits

```rust
{{#include ../../../src/dsa/_13_greedy.rs:interval_scheduling}}
```

## Rust gotchas

- **`p - min` with `min = i32::MAX`** would overflow, so update `min` **before** subtracting
  (as above) or start from `prices[0]`.
- **`i + nums[i]` in `usize`** can't go negative, but can exceed `len`. Compare, don't index.
- Most greedy bugs are logic bugs, not Rust bugs: test the counterexample first.

## Practice

- [ ] [Best Time to Buy and Sell Stock (121)](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)
- [ ] [Jump Game (55)](https://leetcode.com/problems/jump-game/)
- [ ] [Jump Game II (45)](https://leetcode.com/problems/jump-game-ii/)
- [ ] [Non-overlapping Intervals (435)](https://leetcode.com/problems/non-overlapping-intervals/)
- [ ] [Gas Station (134)](https://leetcode.com/problems/gas-station/)
- [ ] [Partition Labels (763)](https://leetcode.com/problems/partition-labels/)
- [ ] [Assign Cookies (455)](https://leetcode.com/problems/assign-cookies/)
