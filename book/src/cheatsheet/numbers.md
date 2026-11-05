# Numbers, Overflow & Bits

Most Rust interview panics come from here. In **debug** builds (which tests and LeetCode
use), integer overflow **panics**. In release it silently wraps.

| Type | Use for |
|---|---|
| `usize` | indices and lengths (what `len()` returns, what `[]` needs) |
| `i32` | LeetCode's default values |
| `i64` | sums, products, anything that might exceed ~2.1 × 10⁹ |
| `isize` / `i32` | signed offsets like grid directions `(-1, 0)` |

| Method family | Behavior on overflow |
|---|---|
| `checked_*` | returns `None` |
| `saturating_*` | clamps to MIN/MAX |
| `wrapping_*` | wraps around |
| `abs_diff` | no overflow, always unsigned |

## usize pitfalls

```rust
{{#include ../../../src/ds/numbers.rs:usize_pitfalls}}
```

## Overflow & sentinels

```rust
{{#include ../../../src/ds/numbers.rs:overflow}}
```

## Math helpers

```rust
{{#include ../../../src/ds/numbers.rs:math}}
```

## Bit manipulation

```rust
{{#include ../../../src/ds/numbers.rs:bits}}
```
