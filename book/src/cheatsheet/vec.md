# Vec & Slices

`Vec<T>` is the owned, growable array. `&[T]` / `&mut [T]` is a borrowed view into one.
Take slices in function signatures (`fn f(nums: &[i32])`) unless you need to push or pop.

| Operation | Method | Cost |
|---|---|---|
| add/remove at end | `push`, `pop` | O(1) |
| add/remove at index | `insert`, `remove` | O(n) |
| remove, order doesn't matter | `swap_remove` | O(1) |
| membership | `contains` | O(n) |
| search sorted | `binary_search`, `partition_point` | O(log n) |
| sort | `sort` (stable), `sort_unstable` | O(n log n) |

## Creating

```rust
{{#include ../../../src/ds/vec.rs:create}}
```

## Adding & removing

```rust
{{#include ../../../src/ds/vec.rs:add_remove}}
```

## Reading

```rust
{{#include ../../../src/ds/vec.rs:access}}
```

## Modifying in place

```rust
{{#include ../../../src/ds/vec.rs:in_place}}
```

## Searching

```rust
{{#include ../../../src/ds/vec.rs:search}}
```

## Slices, windows & chunks

```rust
{{#include ../../../src/ds/vec.rs:slices}}
```

## Two mutable borrows at once

```rust
{{#include ../../../src/ds/vec.rs:borrow_two}}
```

## 2D grids

```rust
{{#include ../../../src/ds/vec.rs:grid_2d}}
```
