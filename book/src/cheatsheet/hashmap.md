# HashMap

`use std::collections::HashMap;` Average **O(1)** insert, get and remove. Iteration order
is **random**, so sort keys before printing or comparing. Use a `BTreeMap` if you need
sorted order.

| Want | Method | Returns |
|---|---|---|
| add / overwrite | `insert(k, v)` | `Option<V>`: the **old** value |
| read | `get(&k)` | `Option<&V>` |
| modify | `get_mut(&k)` | `Option<&mut V>` |
| insert-or-update | `entry(k).or_insert(v)` | `&mut V` |
| check | `contains_key(&k)` | `bool` |
| delete | `remove(&k)` | `Option<V>` |
| filter in place | `retain(\|k, v\| ...)` | |

## Basics

```rust
{{#include ../../../src/ds/hashmap.rs:basics}}
```

## The Entry API (frequency counts & grouping)

```rust
{{#include ../../../src/ds/hashmap.rs:entry}}
```

## Building & iterating

```rust
{{#include ../../../src/ds/hashmap.rs:build_and_iterate}}
```

## Indexing gotchas

```rust
{{#include ../../../src/ds/hashmap.rs:indexing_gotcha}}
```

## Pattern: "have I seen the complement?" (Two Sum)

```rust
{{#include ../../../src/ds/hashmap.rs:two_sum}}
```
