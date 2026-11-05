# Option, Box, Rc&lt;RefCell&gt;

| Type | Meaning | Where you'll see it |
|---|---|---|
| `Option<T>` | maybe a value | every lookup: `get`, `pop`, `first`, `find` |
| `Box<T>` | single owner, on the heap | linked lists: `Option<Box<ListNode>>` |
| `Rc<T>` | shared owners (refcounted) | trees, graphs with shared nodes |
| `RefCell<T>` | mutate through a shared handle (checked at runtime) | `Rc<RefCell<TreeNode>>` |

## Option tools

```rust
{{#include ../../../src/ds/pointers.rs:option_tools}}
```

## Option by reference

```rust
{{#include ../../../src/ds/pointers.rs:option_refs}}
```

## Linked list node

```rust
{{#include ../../../src/ds/pointers.rs:list_node}}
```

## Tree node

```rust
{{#include ../../../src/ds/pointers.rs:tree_node}}
```

```rust
{{#include ../../../src/ds/pointers.rs:tree_usage}}
```

See [Linked Lists](../patterns/linked_lists.md) and [Trees](../patterns/trees.md) for the
traversal templates.
