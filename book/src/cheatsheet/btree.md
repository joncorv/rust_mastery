# BTreeMap & BTreeSet

`use std::collections::{BTreeMap, BTreeSet};` Same API as `HashMap` / `HashSet`, but
**sorted by key**, with **O(log n)** operations.

Reach for them when you need:

- the **smallest / largest** key while inserting and removing (a heap that also supports `remove(x)`),
- **floor / ceiling**: the nearest key ≤ or ≥ x,
- **deterministic, sorted iteration**.

| Want | Method |
|---|---|
| min / max | `first_key_value()` / `last_key_value()` (map), `first()` / `last()` (set) |
| pop min / max | `pop_first()` / `pop_last()` |
| keys in a range | `range(a..b)` |
| largest ≤ x | `range(..=x).next_back()` |
| smallest ≥ x | `range(x..).next()` |

## BTreeMap

```rust
{{#include ../../../src/ds/btree.rs:btreemap}}
```

## Range queries (floor / ceiling)

```rust
{{#include ../../../src/ds/btree.rs:range_queries}}
```

## Multiset (sorted, with duplicates)

```rust
{{#include ../../../src/ds/btree.rs:multiset}}
```
