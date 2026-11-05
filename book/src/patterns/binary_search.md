# Binary Search

## Triggers

- The input is **sorted**, or **sorted then rotated**.
- You need to find a value, a boundary, or the **first/last** index meeting some condition.
- The target complexity is **O(log n)**, or **O(n log n)** with a check inside the search.
- "Find the **minimum X** such that ..." even with no array at all: binary search
  **on the answer** (e.g. the smallest speed that finishes in time).

## Keywords

| sorted | first / last occurrence | minimum / maximum that satisfies | rotated | peak / valley |
|---|---|---|---|---|

## The big idea: the transition point

Don't memorize five binary search variants. Reframe every problem as an array of
`before, before, before, after, after` and find the **first `after`**. You only have to
define `is_after`:

| Problem | `is_after(x)` |
|---|---|
| exact match / lower bound | `x >= target` |
| upper bound (first greater) | `x > target` |
| first bad version | `is_bad(x)` |
| min speed to finish | `hours_needed(x) <= h` |

```rust
{{#include ../../../src/dsa/_02_binary_search.rs:transition_point}}
```

<details>
<summary>My first attempt (closed range with edge cases)</summary>

```rust
{{#include ../../../src/dsa/_02_binary_search.rs:binary_search}}
```

The four up-front edge checks are what the half-open template gets rid of.
</details>

## Rust toolkit

| Method | Returns |
|---|---|
| `v.binary_search(&x)` | `Ok(i)` if found, `Err(i)` = where it would be inserted |
| `v.partition_point(\|&y\| y < x)` | the transition point, i.e. lower bound |
| `v.binary_search_by_key(&k, \|p\| p.0)` | search sorted structs/tuples by a field |
| `BTreeSet::range(x..).next()` | smallest element ≥ x when the data changes over time |

In an interview, say you'd use `partition_point` in production, then write the loop yourself.

## Rust gotchas

- **`mid - 1` underflows when `mid == 0`.** Half-open `[lo, hi)` with `hi = mid` and
  `lo = mid + 1` never subtracts, so it never underflows.
- **`(lo + hi) / 2` can overflow** with `i32` indices or answer-space searches near
  `i32::MAX`. Write `lo + (hi - lo) / 2`.
- **`nums[0]` / `nums.len() - 1` on an empty slice panics.** Check `is_empty()` first.
- **Infinite loops** happen when a branch can leave `lo` and `hi` unchanged (`lo = mid`
  with `mid` rounding down). The template always shrinks the range.

## Practice

- [ ] [Binary Search (704)](https://leetcode.com/problems/binary-search/)
- [ ] [First Bad Version (278)](https://leetcode.com/problems/first-bad-version/)
- [ ] [Find First and Last Position (34)](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)
- [ ] [Search in Rotated Sorted Array (33)](https://leetcode.com/problems/search-in-rotated-sorted-array/)
- [ ] [Find Peak Element (162)](https://leetcode.com/problems/find-peak-element/)
- [ ] [Koko Eating Bananas (875)](https://leetcode.com/problems/koko-eating-bananas/), binary search on the answer
