# Stacks & Queues

## Triggers

- **Matching / nesting**: brackets, tags, undo, "the most recent unmatched thing".
- **Next greater / smaller element**, stock span, daily temperatures: **monotonic stack**.
- **Expression evaluation**: RPN, calculators, decode string.
- **Process in arrival order**: BFS, task scheduling, sliding-window max (monotonic deque).

## Keywords

| valid parentheses | nested | next greater | evaluate expression | most recent |
|---|---|---|---|---|

## The big idea

- **Stack (LIFO):** the answer depends on the **most recent** unresolved item.
- **Queue (FIFO):** items are handled **in the order they arrived**.
- **Monotonic stack:** keep the stack sorted. Each new item pops everything it "resolves",
  so each element is pushed and popped once: **O(n)** total.

## Rust toolkit

- **Stack** = `Vec`: `push`, `pop`, `last()` to peek.
- **Queue / deque** = `VecDeque`: `push_back`, `pop_front`, `front()`.
- `while let Some(&top) = stack.last() && cond { stack.pop(); }` with let-chains (edition 2024).
- Store **indices** on a monotonic stack, not values, so you can write `result[top]`.
- See the [VecDeque](../cheatsheet/stack_queue.md) cheat sheet.

## Template: matching

```rust
{{#include ../../../src/dsa/_04_stacks_and_queues.rs:valid_parentheses}}
```

## Template: monotonic stack

```rust
{{#include ../../../src/dsa/_04_stacks_and_queues.rs:next_greater}}
```

## Template: expression stack

```rust
{{#include ../../../src/dsa/_04_stacks_and_queues.rs:eval_rpn}}
```

## Rust gotchas

- **`stack.last()` returns `Option<&T>`**, which borrows the stack. Copy it out
  (`.copied()` or `Some(&top)`) before you `pop()`/`push()`.
- **`Vec::remove(0)` is O(n).** A queue must be a `VecDeque`.
- **Popping from an empty stack gives `None`, not a panic.** `stack.pop() != Some(c)` handles
  "closer with nothing open" for free.
- **Operand order**: pop `b` first, then `a`, and compute `a - b`.

## Practice

- [ ] [Valid Parentheses (20)](https://leetcode.com/problems/valid-parentheses/)
- [ ] [Min Stack (155)](https://leetcode.com/problems/min-stack/)
- [ ] [Evaluate Reverse Polish Notation (150)](https://leetcode.com/problems/evaluate-reverse-polish-notation/)
- [ ] [Daily Temperatures (739)](https://leetcode.com/problems/daily-temperatures/)
- [ ] [Decode String (394)](https://leetcode.com/problems/decode-string/)
- [ ] [Implement Queue using Stacks (232)](https://leetcode.com/problems/implement-queue-using-stacks/)
- [ ] [Largest Rectangle in Histogram (84)](https://leetcode.com/problems/largest-rectangle-in-histogram/)
