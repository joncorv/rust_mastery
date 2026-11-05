# Prefix Sums

## Triggers

- **Many range-sum queries** on an array that doesn't change.
- "Count subarrays whose sum is k / divisible by k", **especially with negative numbers**,
  where a sliding window breaks.
- Products / XORs / counts over ranges (same idea, different operation).

## Keywords

| range sum | subarray sum equals | divisible by k | running total | product except self |
|---|---|---|---|---|

## The big idea

`prefix[i]` = sum of the first `i` elements, so **`sum(l..=r) = prefix[r + 1] - prefix[l]`**
in O(1) after O(n) setup.

For "count subarrays summing to k": a subarray ending at `r` sums to `k` exactly when some
earlier prefix equals `sum - k`. Keep a **HashMap of prefix-sum counts**, the same
"have I seen the complement" trick as Two Sum.

## Rust toolkit

- Length **n + 1** with `prefix[0] = 0`, so no special case for ranges starting at 0.
- `iter().scan(0, |s, &x| { *s += x; Some(*s) })` for a quick running total.
- `HashMap::from([(0, 1)])` seeds the map with the empty prefix.
- `i64` for sums of many `i32`s.

## Template: build + range query

```rust
{{#include ../../../src/dsa/_15_prefix_sums.rs:prefix}}
```

## Template: prefix sums + hash map

```rust
{{#include ../../../src/dsa/_15_prefix_sums.rs:subarray_sum_k}}
```

## Template: prefix and suffix passes

```rust
{{#include ../../../src/dsa/_15_prefix_sums.rs:product_except_self}}
```

## Rust gotchas

- **Off-by-one between `prefix` and `nums` indices** is the #1 bug. Write the formula as a
  comment before coding.
- **Overflow**: sums of 10⁵ values up to 10⁹ exceed `i32`. Use `i64`.
- **Negative modulo**: for "divisible by k", `sum % k` can be negative. Use `sum.rem_euclid(k)`.
- **Seeding the map** with `(0, 1)` is what counts subarrays starting at index 0. Forget it
  and you're off by those.

## Practice

- [ ] [Range Sum Query - Immutable (303)](https://leetcode.com/problems/range-sum-query-immutable/)
- [ ] [Find Pivot Index (724)](https://leetcode.com/problems/find-pivot-index/)
- [ ] [Subarray Sum Equals K (560)](https://leetcode.com/problems/subarray-sum-equals-k/)
- [ ] [Product of Array Except Self (238)](https://leetcode.com/problems/product-of-array-except-self/)
- [ ] [Subarray Sums Divisible by K (974)](https://leetcode.com/problems/subarray-sums-divisible-by-k/)
- [ ] [Contiguous Array (525)](https://leetcode.com/problems/contiguous-array/)
- [ ] [Range Sum Query 2D - Immutable (304)](https://leetcode.com/problems/range-sum-query-2d-immutable/)
