# Graphs

## Triggers

- Things connected by **relationships**: friends, flights, dependencies, word ladders.
- **Reachability / connected components / count groups**: DFS or BFS.
- **Fewest steps** in an unweighted graph: BFS. **Weighted**: Dijkstra ([Heaps](heaps.md)).
- **Ordering with prerequisites**: [Topological Sort](topological_sort.md).
- Grids are graphs too ([Grids](grids.md)).

## Keywords

| connected | network | path / route | neighbors | shortest / fewest steps |
|---|---|---|---|---|

## The big idea

1. **Build an adjacency list** from whatever the input gives you.
2. **Traverse with a visited set**: DFS (stack / recursion) for "can I reach / how many
   groups", BFS (queue) for "how few steps".
3. Complexity is **O(V + E)** for both.

## Rust toolkit

| Situation | Representation |
|---|---|
| nodes are `0..n` | `Vec<Vec<usize>>` + `vec![false; n]` visited |
| nodes are strings / sparse ids | `HashMap<&str, Vec<&str>>` + `HashSet` visited |
| weighted | `Vec<Vec<(usize, u64)>>` |

## Template: adjacency list

```rust
{{#include ../../../src/dsa/_08_graphs.rs:build_adj}}
```

## Template: BFS (shortest path)

```rust
{{#include ../../../src/dsa/_08_graphs.rs:bfs}}
```

## Template: iterative DFS

```rust
{{#include ../../../src/dsa/_08_graphs.rs:dfs}}
```

## Template: recursive DFS

```rust
{{#include ../../../src/dsa/_08_graphs.rs:dfs_recursive}}
```

## Rust gotchas

- **Mark visited when you ENQUEUE, not when you dequeue.** Otherwise a node gets queued many
  times (correct answer, but much slower).
- **Iterating `adj[u]` while mutating `adj`** won't compile. You almost never need to; mutate
  `seen` / `dist`, which are separate Vecs.
- **Recursive closures don't exist.** Use a nested `fn`, or an explicit stack (which also
  avoids stack overflow on long paths).
- **`HashMap<&str, Vec<&str>>` built from input strings** borrows the input. That's fine, but the
  graph can't outlive it.

## Practice

- [ ] [Find if Path Exists in Graph (1971)](https://leetcode.com/problems/find-if-path-exists-in-graph/)
- [ ] [Number of Connected Components (323)](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/)
- [ ] [Clone Graph (133)](https://leetcode.com/problems/clone-graph/)
- [ ] [Rotting Oranges (994)](https://leetcode.com/problems/rotting-oranges/)
- [ ] [Word Ladder (127)](https://leetcode.com/problems/word-ladder/)
- [ ] [Network Delay Time (743)](https://leetcode.com/problems/network-delay-time/), Dijkstra
- [ ] [Is Graph Bipartite? (785)](https://leetcode.com/problems/is-graph-bipartite/)
