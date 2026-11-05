# Sliding Windows

## Triggers

- **Contiguous** subarray or substring.
- "**Longest / shortest / count** subarray such that ..." with a condition you can update
  incrementally as elements enter and leave.
- The naive solution checks every (start, end) pair: **O(n²)** → want **O(n)**.

## Keywords

| contiguous | substring / subarray | longest / shortest | at most k | window of size k |
|---|---|---|---|---|

## The big idea

Two pointers moving **the same direction**: `r` adds an element, `l` removes one. Each moves
at most n times, so the whole thing is **O(n)**. Keep window state (sum, counts, distinct
count) updated in O(1) per move.

| Window type | Shape |
|---|---|
| **fixed size k** | add `nums[i]`, subtract `nums[i - k]` |
| **longest valid** | grow `r`; `while invalid { shrink l }`; record **after** the while |
| **shortest valid** | grow `r`; `while valid { record; shrink l }` |

**When it fails:** if shrinking the window doesn't move it back toward valid (e.g.
**negative numbers** in a sum), use [Prefix Sums](prefix_sums.md) + a map instead.

## Rust toolkit

- Byte strings: `s.as_bytes()` + `[usize; 256]` or `[usize; 26]` counts.
- `r - l + 1` for the window length (`usize`, never negative since `l <= r + 1`).
- `nums.windows(k)` for read-only fixed windows (but recomputing the sum each time is O(nk)).

## Template: fixed window

```rust
{{#include ../../../src/dsa/_10_sliding_window.rs:fixed_window}}
```

## Template: longest valid window

```rust
{{#include ../../../src/dsa/_10_sliding_window.rs:variable_window}}
```

## Template: shortest valid window

```rust
{{#include ../../../src/dsa/_10_sliding_window.rs:min_window}}
```

## Rust gotchas

- **`s.chars().nth(i)` inside the loop is O(n)**, which quietly makes the whole thing O(n²).
  Convert to bytes or `Vec<char>` once.
- **`usize::MAX` sentinel for "no answer"**: convert with `.then_some(best)` at the end.
- **`nums[..k]` panics if `k > len`.** Guard first.

## Practice

- [ ] [Maximum Average Subarray I (643)](https://leetcode.com/problems/maximum-average-subarray-i/)
- [ ] [Longest Substring Without Repeating Characters (3)](https://leetcode.com/problems/longest-substring-without-repeating-characters/)
- [ ] [Minimum Size Subarray Sum (209)](https://leetcode.com/problems/minimum-size-subarray-sum/)
- [ ] [Longest Repeating Character Replacement (424)](https://leetcode.com/problems/longest-repeating-character-replacement/)
- [ ] [Permutation in String (567)](https://leetcode.com/problems/permutation-in-string/)
- [ ] [Minimum Window Substring (76)](https://leetcode.com/problems/minimum-window-substring/)
- [ ] [Sliding Window Maximum (239)](https://leetcode.com/problems/sliding-window-maximum/), monotonic deque
