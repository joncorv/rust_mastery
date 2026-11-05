# Trees

## Triggers

- The input is a `TreeNode` (binary tree or BST), or a hierarchy (org chart, file system).
- **Depth / height / diameter / path sums**: DFS.
- **Level-by-level / "closest to root" / right-side view**: BFS.
- **BST**: in-order traversal is sorted; search is O(height).

## Keywords

| depth / height | level order | ancestor | BST | path sum |
|---|---|---|---|---|

## The big idea

Pick the traversal by **which direction information flows**:

| Flow | Shape | Examples |
|---|---|---|
| **bottom-up** (children → parent) | return a value from each call | depth, diameter, balanced? |
| **top-down** (parent → children) | pass parameters down | valid BST bounds, path sum |
| **level by level** | BFS with a queue | level order, right side view, min depth |

## Rust toolkit

```rust
{{#include ../../../src/ds/pointers.rs:tree_node}}
```

| Tool | Why |
|---|---|
| `node.borrow()` / `borrow_mut()` | read / write through the `RefCell` |
| `Rc::clone(&rc)` / `opt.clone()` | another handle to the same node (cheap refcount bump) |
| `let Some(n) = root else { return 0 };` | the base case in one line |
| `&n.left` | recurse on `&Tree` to avoid cloning `Rc`s at all |

## Template: DFS (bottom-up)

```rust
{{#include ../../../src/dsa/_07_trees.rs:max_depth}}
```

## Template: DFS (top-down, passing bounds)

```rust
{{#include ../../../src/dsa/_07_trees.rs:is_valid_bst}}
```

## Template: BFS (level order)

```rust
{{#include ../../../src/dsa/_07_trees.rs:level_order}}
```

## Template: mutating a tree

```rust
{{#include ../../../src/dsa/_07_trees.rs:invert_tree}}
```

## Rust gotchas

- **`Option<Rc<RefCell<TreeNode>>>` is a mouthful.** Write `type Tree = ...;` at the top of
  your solution. Interviewers appreciate it.
- **`borrow()` and `borrow_mut()` on the same node at the same time panics at runtime**
  ("already borrowed"). Keep borrows short and drop them before recursing into code that mutates.
- **The `Ref` from `borrow()` is a temporary.** `root.as_ref().unwrap().borrow().left.clone()`
  is fine, but you can't return a reference into it.
- **Two mutable fields at once**: `let n = &mut *node.borrow_mut();` then
  `std::mem::swap(&mut n.left, &mut n.right)`.
- **BST validation needs bounds, not just parent/child checks**, and `i32::MIN`/`MAX` can be
  real values, so use `Option<i32>` (or `i64`) bounds.

## Practice

- [ ] [Maximum Depth of Binary Tree (104)](https://leetcode.com/problems/maximum-depth-of-binary-tree/)
- [ ] [Invert Binary Tree (226)](https://leetcode.com/problems/invert-binary-tree/)
- [ ] [Binary Tree Level Order Traversal (102)](https://leetcode.com/problems/binary-tree-level-order-traversal/)
- [ ] [Validate Binary Search Tree (98)](https://leetcode.com/problems/validate-binary-search-tree/)
- [ ] [Diameter of Binary Tree (543)](https://leetcode.com/problems/diameter-of-binary-tree/)
- [ ] [Lowest Common Ancestor of a BST (235)](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)
- [ ] [Binary Tree Right Side View (199)](https://leetcode.com/problems/binary-tree-right-side-view/)
- [ ] [Kth Smallest Element in a BST (230)](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)
