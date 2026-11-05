# BinaryHeap

`use std::collections::BinaryHeap;` A **max-heap**. Use `std::cmp::Reverse` for a min-heap.

| Operation | Method | Cost |
|---|---|---|
| add | `push(x)` | O(log n) |
| remove top | `pop()` → `Option<T>` | O(log n) |
| look at top | `peek()` → `Option<&T>` | O(1) |
| build from Vec | `BinaryHeap::from(v)` / `v.into()` | O(n) |
| drain in order | `into_sorted_vec()` (ascending) | O(n log n) |

## Max-heap

```rust
{{#include ../../../src/ds/heap.rs:max_heap}}
```

## Min-heap with `Reverse`

```rust
{{#include ../../../src/ds/heap.rs:min_heap}}
```

## Top k

```rust
{{#include ../../../src/ds/heap.rs:top_k}}
```

## Custom ordering

```rust
{{#include ../../../src/ds/heap.rs:custom_ord}}
```
