# Two Pointers

## Triggers

- The input is an **array, string, or linked list**.
- You must modify it **in place** with **O(1) extra memory**.
- The input is **sorted** (or you can sort it).
- The naive solution is **O(n²)** nested loops and you want **O(n)**.

## Keywords

| palindrome | reverse | swap | merge | partition |
|---|---|---|---|---|

## Movement patterns

| Pattern | Pointers | Classic problems |
|---|---|---|
| **Inward** | `left = 0`, `right = len - 1`, move toward each other | reverse, palindrome, two-sum on sorted |
| **Slow / fast** (read/write) | both start at 0, `fast` scans, `slow` marks where to write | move zeroes, remove duplicates, partition |
| **Parallel** | one pointer per array | merge two sorted arrays, intersection |

## Rust toolkit

- `nums.swap(i, j)`: swap in place. You can't hold two `&mut nums[..]` at once.
- `&mut [T]` instead of `&mut Vec<T>` in signatures, since you never need to resize.
- `s.as_bytes()` / `s.chars().collect::<Vec<char>>()` for O(1) string indexing.
- `nums.split_at_mut(mid)` when you need two disjoint mutable halves.

## Template: inward pointers

```rust
{{#include ../../../src/dsa/_01_two_pointers.rs:reverse_string}}
```

## Template: read/write pointers

```rust
{{#include ../../../src/dsa/_01_two_pointers.rs:move_zeroes_write_ptr}}
```

<details>
<summary>My first attempt at Move Zeroes (case-by-case version)</summary>

```rust
{{#include ../../../src/dsa/_01_two_pointers.rs:move_zeroes}}
```

It works, but it has three cases to explain out loud. The read/write version has one.
</details>

## Rust gotchas

- **`len() - 1` on an empty slice panics** (usize underflow). Guard with `is_empty()` or
  start with `right = len` and use `right - 1` only inside the loop.
- **`right -= 1` when `right == 0` panics.** With `while left < right`, `right` is always ≥ 1
  when you decrement it, so the loop condition protects you.
- Pick the loop condition deliberately: `left < right` stops before the pointers meet
  (fine for swapping). Use `left <= right` when the middle element still needs checking.

## Practice

- [ ] [Valid Palindrome (125)](https://leetcode.com/problems/valid-palindrome/)
- [ ] [Two Sum II, sorted input (167)](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)
- [ ] [Remove Duplicates from Sorted Array (26)](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)
- [ ] [Merge Sorted Array (88)](https://leetcode.com/problems/merge-sorted-array/)
- [ ] [Sort Colors (75)](https://leetcode.com/problems/sort-colors/)
- [ ] [Container With Most Water (11)](https://leetcode.com/problems/container-with-most-water/)
- [ ] [3Sum (15)](https://leetcode.com/problems/3sum/)
