# Iterators

Iterator chains are lazy. Nothing runs until a **consumer** (`collect`, `sum`, `count`,
`for`, ...) pulls values. `collect` needs a type: annotate the variable or use `::<Vec<_>>`.

| Adapters (lazy) | Consumers (run it) |
|---|---|
| `map`, `filter`, `filter_map`, `flat_map`, `flatten` | `collect`, `sum`, `product`, `count` |
| `enumerate`, `zip`, `rev`, `skip`, `take`, `step_by` | `min`, `max`, `min_by_key`, `max_by_key` |
| `take_while`, `skip_while`, `scan`, `peekable` | `any`, `all`, `find`, `position`, `fold` |
| `copied`, `cloned`, `chain`, `windows` (slices) | `last`, `nth`, `partition`, `unzip` |

## iter vs iter_mut vs into_iter

```rust
{{#include ../../../src/ds/iterators.rs:iter_kinds}}
```

## Transforming

```rust
{{#include ../../../src/ds/iterators.rs:transform}}
```

## Skipping & taking

```rust
{{#include ../../../src/ds/iterators.rs:slicing_iters}}
```

## Reducing to one value

```rust
{{#include ../../../src/ds/iterators.rs:reduce}}
```

## min/max gotchas

```rust
{{#include ../../../src/ds/iterators.rs:max_by_gotchas}}
```

## Grouping runs (peekable & chunk_by)

```rust
{{#include ../../../src/ds/iterators.rs:peekable}}
```
