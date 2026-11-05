# VecDeque (Stacks & Queues)

- **Stack** → plain `Vec<T>`: `push` / `pop` / `last`.
- **Queue or deque** → `std::collections::VecDeque<T>`: O(1) at both ends.

| | Stack (`Vec`) | Queue (`VecDeque`) |
|---|---|---|
| add | `push(x)` | `push_back(x)` (or `push_front`) |
| remove | `pop()` | `pop_front()` (or `pop_back`) |
| peek | `last()` | `front()` / `back()` |
| empty? | `is_empty()` | `is_empty()` |

## Stack

```rust
{{#include ../../../src/ds/stack_queue.rs:stack}}
```

## Queue / deque

```rust
{{#include ../../../src/ds/stack_queue.rs:queue}}
```

## Extras

```rust
{{#include ../../../src/ds/stack_queue.rs:deque_extras}}
```
