# Sorting

## Triggers

- The order of the input doesn't matter to the answer, so **sort first** for O(n log n)
  and unlock two pointers, binary search, or greedy.
- **Intervals**: merge, insert, overlap, meeting rooms.
- "Arrange by **custom rule**" (largest number, sort by frequency, people by height).
- The values sit in a **small known range**: counting sort, O(n).

## Keywords

| intervals | overlap | k-th smallest | arrange / order by | custom comparator |
|---|---|---|---|---|

## The big idea

Sorting is usually a **preprocessing step**, not the solution. Ask: *if this were sorted,
would the problem become easy?* If you sort, say so and account for the O(n log n).

## Rust toolkit

| Want | Use |
|---|---|
| default order | `v.sort()` (stable) / `v.sort_unstable()` (faster, no alloc) |
| by one field | `v.sort_by_key(\|x\| x.1)` |
| descending | `v.sort_by_key(\|&x\| Reverse(x))` or `v.sort_by(\|a, b\| b.cmp(a))` |
| multi-key | `a.1.cmp(&b.1).then_with(\|\| a.0.cmp(&b.0))` |
| floats | `v.sort_by(\|a, b\| a.total_cmp(b))` |
| k-th smallest, O(n) | `v.select_nth_unstable(k)` |

See the [Sorting & Ordering](../cheatsheet/sorting.md) cheat sheet.

## Template: sort then sweep (intervals)

```rust
{{#include ../../../src/dsa/_17_sorting.rs:merge_intervals}}
```

## Template: custom comparator

```rust
{{#include ../../../src/dsa/_17_sorting.rs:sort_by_frequency}}
```

## Template: counting sort

```rust
{{#include ../../../src/dsa/_17_sorting.rs:counting_sort}}
```

## Rust gotchas

- **`sort` mutates in place and returns `()`.** `let s = v.sort();` is a bug. Clone first if
  you need the original.
- **`f64` doesn't implement `Ord`**, so `.sort()` won't compile. Use `total_cmp`.
- **`sort_by_key` with a key that borrows** (`|s| &s.name`) won't compile. Use
  `sort_by(|a, b| a.name.cmp(&b.name))` or `sort_by_cached_key(|s| s.name.clone())`.
- **Stability matters for multi-pass sorts.** `sort_unstable` can reorder equal keys.
- Know the cost: Rust's `sort` is O(n log n) worst case and O(n) on already-sorted runs.

## Practice

- [ ] [Merge Intervals (56)](https://leetcode.com/problems/merge-intervals/)
- [ ] [Meeting Rooms II (253)](https://leetcode.com/problems/meeting-rooms-ii/)
- [ ] [Sort Colors (75)](https://leetcode.com/problems/sort-colors/)
- [ ] [Largest Number (179)](https://leetcode.com/problems/largest-number/)
- [ ] [Sort Characters By Frequency (451)](https://leetcode.com/problems/sort-characters-by-frequency/)
- [ ] [Kth Largest Element (215)](https://leetcode.com/problems/kth-largest-element-in-an-array/)
