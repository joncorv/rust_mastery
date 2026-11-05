//! Graphs. Notes live in book/src/patterns/graphs.md.

use std::collections::VecDeque;

// ANCHOR: build_adj
/// Edge list -> adjacency list. Nodes are 0..n, so Vec<Vec<usize>> beats a HashMap.
pub fn build_adj(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u); // drop this line for a directed graph
    }
    adj
}
// ANCHOR_END: build_adj

// ANCHOR: bfs
/// BFS: shortest path (in edges) from `start` to every node. None = unreachable.
pub fn bfs_dist(adj: &[Vec<usize>], start: usize) -> Vec<Option<usize>> {
    let mut dist = vec![None; adj.len()];
    dist[start] = Some(0);
    let mut q = VecDeque::from([start]);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if dist[v].is_none() {
                // mark visited when ENQUEUED, not when popped
                dist[v] = Some(dist[u].unwrap() + 1);
                q.push_back(v);
            }
        }
    }
    dist
}
// ANCHOR_END: bfs

// ANCHOR: dfs
/// Iterative DFS with an explicit stack: count connected components.
pub fn count_components(adj: &[Vec<usize>]) -> usize {
    let mut seen = vec![false; adj.len()];
    let mut count = 0;
    for s in 0..adj.len() {
        if seen[s] {
            continue;
        }
        count += 1;
        seen[s] = true;
        let mut stack = vec![s];
        while let Some(u) = stack.pop() {
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    stack.push(v);
                }
            }
        }
    }
    count
}
// ANCHOR_END: dfs

// ANCHOR: dfs_recursive
/// Recursive DFS: closures can't call themselves, so use a nested fn and pass state in.
pub fn has_path(adj: &[Vec<usize>], from: usize, to: usize) -> bool {
    fn dfs(adj: &[Vec<usize>], u: usize, to: usize, seen: &mut [bool]) -> bool {
        if u == to {
            return true;
        }
        seen[u] = true;
        adj[u].iter().any(|&v| !seen[v] && dfs(adj, v, to, seen))
    }
    dfs(adj, from, to, &mut vec![false; adj.len()])
}
// ANCHOR_END: dfs_recursive

#[test]
fn test_graphs() {
    // 0-1-2   3-4   5
    let adj = build_adj(6, &[(0, 1), (1, 2), (3, 4)]);
    assert_eq!(bfs_dist(&adj, 0), [Some(0), Some(1), Some(2), None, None, None]);
    assert_eq!(count_components(&adj), 3);
    assert!(has_path(&adj, 0, 2));
    assert!(!has_path(&adj, 0, 4));
}
