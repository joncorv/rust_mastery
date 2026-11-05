//! Topological Sort. Notes live in book/src/patterns/topological_sort.md.

use std::collections::VecDeque;

// ANCHOR: kahn
/// Kahn's algorithm. Edge (u, v) means "u before v". Returns None if there's a cycle.
pub fn topo_sort(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    let mut indegree = vec![0; n];
    for &(u, v) in edges {
        adj[u].push(v);
        indegree[v] += 1;
    }
    // start with everything that has no prerequisites
    let mut q: VecDeque<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
    let mut order = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                q.push_back(v);
            }
        }
    }
    // nodes left over were stuck in a cycle
    (order.len() == n).then_some(order)
}
// ANCHOR_END: kahn

// ANCHOR: course_schedule
/// 207. Course Schedule. LeetCode gives [course, prereq], which is the REVERSE of (u before v).
pub fn can_finish(num_courses: usize, prerequisites: &[[usize; 2]]) -> bool {
    let edges: Vec<(usize, usize)> = prerequisites.iter().map(|&[course, pre]| (pre, course)).collect();
    topo_sort(num_courses, &edges).is_some()
}
// ANCHOR_END: course_schedule

#[test]
fn test_topo_sort() {
    let order = topo_sort(4, &[(0, 1), (0, 2), (1, 3), (2, 3)]).unwrap();
    let pos = |x| order.iter().position(|&y| y == x).unwrap();
    assert!(pos(0) < pos(1) && pos(0) < pos(2) && pos(1) < pos(3) && pos(2) < pos(3));
    assert_eq!(topo_sort(2, &[(0, 1), (1, 0)]), None);
    assert!(can_finish(2, &[[1, 0]]));
    assert!(!can_finish(2, &[[1, 0], [0, 1]]));
}
