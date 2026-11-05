# Linked Lists

## Triggers

- The input is a `ListNode`. That's the only real trigger.
- Reverse, merge, reorder, find the middle / k-th from end, detect a cycle.

## Keywords

| reverse | merge | middle | cycle | k-th from end |
|---|---|---|---|---|

## The big idea

Almost every list problem uses one of three tricks:

1. **Dummy head**: a fake node in front so "insert at head" isn't a special case.
2. **Slow / fast pointers**: the middle, the k-th from end, cycle detection.
3. **Reverse in place**: `prev / cur / next` rewiring.

In Rust, linked lists are the **hardest** topic because of ownership. Learn the two shapes
below by heart: **owning walk** (consume and rebuild) and **borrowing walk** (read only).

## Rust toolkit

```rust
{{#include ../../../src/ds/pointers.rs:list_node}}
```

| Tool | Why |
|---|---|
| `node.next.take()` | move the next node out, leaving `None` (detach) |
| `opt.as_deref()` | `&Option<Box<ListNode>>` → `Option<&ListNode>` for read-only walks |
| `opt.as_deref_mut()` | same, mutable |
| `opt.insert(node)` | store and get back `&mut` to it (tail cursor) |
| `a.or(b)` | "whichever is non-empty" |

## Template: reverse (owning walk)

```rust
{{#include ../../../src/dsa/_06_linked_lists.rs:reverse_list}}
```

## Template: slow / fast (borrowing walk)

```rust
{{#include ../../../src/dsa/_06_linked_lists.rs:middle_node}}
```

## Template: dummy head + tail cursor

```rust
{{#include ../../../src/dsa/_06_linked_lists.rs:merge_two}}
```

## Rust gotchas

- **You can't have two `&mut` into the list at once**, so the "fast/slow with mutation"
  versions from other languages don't translate. Common workarounds: count the length
  first, then walk again. Or collect values into a `Vec`, solve, and rebuild (say so
  in the interview: it's O(n) space).
- **Cycle detection isn't possible with `Box`.** A cycle needs `Rc`, which is why LeetCode's
  Rust version of 141 doesn't exist. Know Floyd's algorithm conceptually.
- **`while let Some(node) = cur` moves `cur`** if `cur` is an owned `Option`. Iterate over
  `&cur`, or reassign it inside the loop.
- **Mutating through `&mut Option<Box<_>>` cursors** fights the borrow checker. Prefer
  `take()` + rebuild over in-place pointer surgery.

## Practice

- [ ] [Reverse Linked List (206)](https://leetcode.com/problems/reverse-linked-list/)
- [ ] [Merge Two Sorted Lists (21)](https://leetcode.com/problems/merge-two-sorted-lists/)
- [ ] [Middle of the Linked List (876)](https://leetcode.com/problems/middle-of-the-linked-list/)
- [ ] [Remove Nth Node From End (19)](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)
- [ ] [Palindrome Linked List (234)](https://leetcode.com/problems/palindrome-linked-list/)
- [ ] [Reorder List (143)](https://leetcode.com/problems/reorder-list/)
- [ ] [Add Two Numbers (2)](https://leetcode.com/problems/add-two-numbers/)
