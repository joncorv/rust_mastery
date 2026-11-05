# Topological Sort

## Triggers

- **Prerequisites / dependencies**: courses, build steps, package installs, task ordering.
- "Find **an order** such that every A comes before B."
- "Is it **possible**?" = does the dependency graph have a **cycle**?

## Keywords

| prerequisite | depends on | order / schedule | before / after | cycle |
|---|---|---|---|---|

## The big idea

**Kahn's algorithm** (BFS on in-degrees):

1. Count each node's **in-degree** (number of prerequisites).
2. Queue every node with in-degree 0 (nothing blocking it).
3. Pop a node, append it to the order, and decrement its neighbors' in-degrees. Any
   neighbor that hits 0 joins the queue.
4. If the order has fewer than `n` nodes, the rest are stuck in a **cycle**.

O(V + E). Prefer it to the DFS version in interviews: no recursion, and cycle detection is free.

## Rust toolkit

- `vec![vec![]; n]` adjacency + `vec![0; n]` in-degrees.
- `(0..n).filter(|&i| indegree[i] == 0).collect::<VecDeque<_>>()` to seed the queue.
- `(order.len() == n).then_some(order)` turns "is there a cycle" into an `Option`.

## Template: Kahn's algorithm

```rust
{{#include ../../../src/dsa/_14_topological_sort.rs:kahn}}
```

## Template: adapting LeetCode's edge format

```rust
{{#include ../../../src/dsa/_14_topological_sort.rs:course_schedule}}
```

## Rust gotchas

- **Read the edge direction carefully.** LeetCode's `[course, prereq]` means
  prereq → course. Getting it backwards gives you a reversed order (still "valid" for the
  cycle check, wrong for the order).
- **Nodes given as strings** (alien dictionary, recipes): map them to `0..n` indices with a
  `HashMap<&str, usize>` first, then use the `Vec` version.
- **Multiple valid orders exist.** Tests should check the *constraints*, not one exact order.

## Practice

- [ ] [Course Schedule (207)](https://leetcode.com/problems/course-schedule/)
- [ ] [Course Schedule II (210)](https://leetcode.com/problems/course-schedule-ii/)
- [ ] [Find All Possible Recipes (2115)](https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/)
- [ ] [Parallel Courses (1136)](https://leetcode.com/problems/parallel-courses/)
- [ ] [Alien Dictionary (269)](https://leetcode.com/problems/alien-dictionary/)
