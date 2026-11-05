# HashSet

`use std::collections::HashSet;` Average **O(1)** insert, contains and remove. No duplicates,
no order. Use `BTreeSet` for sorted order.

| Want | Method | Returns |
|---|---|---|
| add | `insert(x)` | `bool`: **true if new** |
| check | `contains(&x)` | `bool` |
| delete | `remove(&x)` | `bool`: true if it was there |
| filter in place | `retain(\|x\| ...)` | |
| set algebra | `intersection`, `union`, `difference`, `symmetric_difference` | lazy iterators of `&T` |
| relations | `is_subset`, `is_superset`, `is_disjoint` | `bool` |

## Basics

```rust
{{#include ../../../src/ds/hashset.rs:basics}}
```

## Set operations

```rust
{{#include ../../../src/ds/hashset.rs:set_ops}}
```

## Filtering

```rust
{{#include ../../../src/ds/hashset.rs:filter_out}}
```

## What can be a key?

```rust
{{#include ../../../src/ds/hashset.rs:hashable_keys}}
```
