# Sorting & Ordering

| Want | Method |
|---|---|
| ascending, stable | `v.sort()` |
| ascending, faster | `v.sort_unstable()` |
| by a key | `v.sort_by_key(\|x\| key)` |
| custom compare | `v.sort_by(\|a, b\| a.cmp(b))` |
| descending | `sort_by(\|a, b\| b.cmp(a))` or `sort_by_key(\|&x\| Reverse(x))` |
| tie-breakers | `.then(..)` / `.then_with(\|\| ..)` on an `Ordering` |
| floats | `sort_by(\|a, b\| a.total_cmp(b))` |
| k-th smallest, O(n) avg | `select_nth_unstable(k)` |
| already sorted? | `is_sorted()` |

## Basics

```rust
{{#include ../../../src/ds/sorting.rs:sort_basics}}
```

## Multiple keys

```rust
{{#include ../../../src/ds/sorting.rs:multi_key}}
```

## Sort + sweep (intervals)

```rust
{{#include ../../../src/ds/sorting.rs:intervals}}
```

## Ordering, floats & selection

```rust
{{#include ../../../src/ds/sorting.rs:ordering_and_floats}}
```
