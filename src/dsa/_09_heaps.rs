//! Heaps. Notes live in book/src/patterns/heaps.md.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

// ANCHOR: kth_largest
/// 215. Kth Largest Element: min-heap of size k; its top is the answer. O(n log k)
pub fn kth_largest(nums: &[i32], k: usize) -> Option<i32> {
    let mut heap = BinaryHeap::with_capacity(k + 1);
    for &x in nums {
        heap.push(Reverse(x));
        if heap.len() > k {
            heap.pop(); // evict the smallest; the k largest remain
        }
    }
    if heap.len() < k { None } else { heap.peek().map(|&Reverse(x)| x) }
}
// ANCHOR_END: kth_largest

// ANCHOR: merge_k_sorted
/// 23-style K-way merge: heap holds one (value, list, index) cursor per list.
pub fn merge_k_sorted(lists: &[Vec<i32>]) -> Vec<i32> {
    let mut pq = BinaryHeap::new();
    for (i, list) in lists.iter().enumerate() {
        if let Some(&x) = list.first() {
            pq.push(Reverse((x, i, 0)));
        }
    }
    let mut out = Vec::new();
    while let Some(Reverse((x, i, j))) = pq.pop() {
        out.push(x);
        if let Some(&next) = lists[i].get(j + 1) {
            pq.push(Reverse((next, i, j + 1)));
        }
    }
    out
}
// ANCHOR_END: merge_k_sorted

// ANCHOR: dijkstra
/// Dijkstra: shortest weighted distances. adj[u] = [(v, weight), ...]
pub fn dijkstra(adj: &[Vec<(usize, u64)>], src: usize) -> Vec<u64> {
    let mut dist = vec![u64::MAX; adj.len()];
    dist[src] = 0;
    let mut pq = BinaryHeap::from([Reverse((0u64, src))]); // (dist, node): dist FIRST
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue; // stale entry: we already found a shorter path
        }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(Reverse((nd, v)));
            }
        }
    }
    dist
}
// ANCHOR_END: dijkstra

#[test]
fn test_heaps() {
    assert_eq!(kth_largest(&[3, 2, 1, 5, 6, 4], 2), Some(5));
    assert_eq!(kth_largest(&[1], 2), None);
    assert_eq!(merge_k_sorted(&[vec![1, 4, 5], vec![1, 3, 4], vec![2, 6], vec![]]), [1, 1, 2, 3, 4, 4, 5, 6]);
    // 0 -4-> 1, 0 -1-> 2, 2 -2-> 1, 1 -5-> 3
    let adj = vec![vec![(1, 4), (2, 1)], vec![(3, 5)], vec![(1, 2)], vec![]];
    assert_eq!(dijkstra(&adj, 0), [0, 3, 1, 8]);
}
