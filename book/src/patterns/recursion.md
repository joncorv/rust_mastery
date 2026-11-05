# Recursion

## Triggers

- The problem is **defined in terms of a smaller version of itself** (factorial, power, fib).
- The data is **recursive**: trees, nested lists, JSON, file systems.
- You can split the input in half (**divide & conquer**: merge sort, fast power).
- It's the foundation for [Backtracking](backtracking.md), [Trees](trees.md), and top-down
  [DP](dynamic_programming.md).

## Keywords

| nested | recursive definition | divide and conquer | self-similar | depth |
|---|---|---|---|---|

## The big idea

Every recursive function answers three questions:

1. **Base case**: what's the smallest input I can answer directly?
2. **Recursive case**: assume the call works on a smaller input (the "leap of faith").
3. **Combine**: how do I build my answer from the smaller answer?

Complexity ≈ **(number of calls) × (work per call)**. Space = **max recursion depth**.

## Rust toolkit

- **Slice patterns**: `[first, rest @ ..]` and `[first, mid @ .., last]` split slices with no indexing.
- **Nested `fn` helpers**: an inner `fn go(...)` that takes `&mut` state (a memo or an output Vec).
- **Recursive types need indirection**: `Box<T>`, `Vec<T>`, or `Option<Box<T>>`.

## Template: divide & conquer

```rust
{{#include ../../../src/dsa/_05_recursion.rs:fast_pow}}
```

## Template: slice patterns

```rust
{{#include ../../../src/dsa/_05_recursion.rs:slice_patterns}}
```

## Template: recursive data + `&mut` accumulator

```rust
{{#include ../../../src/dsa/_05_recursion.rs:accumulator}}
```

## Rust gotchas

- **Closures can't call themselves.** Use a nested `fn` and pass state in explicitly
  (the inner `fn` can't capture outer variables either).
- **Rust has no tail-call optimization.** The main thread's stack is ~8 MB, and ~10⁵ deep
  frames can overflow. Convert deep recursion (linked lists, flood fill) to a loop or explicit stack.
- **Returning a new `Vec` from every call** is O(n) copying per level. Prefer a `&mut Vec` accumulator.
- **`struct Node { next: Node }` won't compile** (infinite size). It must be `Option<Box<Node>>`.

## Practice

- [ ] [Pow(x, n) (50)](https://leetcode.com/problems/powx-n/)
- [ ] [Fibonacci Number (509)](https://leetcode.com/problems/fibonacci-number/)
- [ ] [Reverse String recursively (344)](https://leetcode.com/problems/reverse-string/)
- [ ] [Flatten Nested List Iterator (341)](https://leetcode.com/problems/flatten-nested-list-iterator/)
- [ ] [Sort an Array with merge sort (912)](https://leetcode.com/problems/sort-an-array/)
- [ ] [K-th Symbol in Grammar (779)](https://leetcode.com/problems/k-th-symbol-in-grammar/)
