# Heaps

## Triggers

- **Top k / k-th largest / k closest / k most frequent.**
- Repeatedly take the **current min or max** while new items keep arriving.
- **Merge k sorted** lists or streams.
- **Shortest weighted path** (Dijkstra), scheduling, "median of a stream" (two heaps).

## Keywords

| top k | k-th largest / smallest | closest | merge k | schedule / priority |
|---|---|---|---|---|

## The big idea

A heap gives **O(1) peek and O(log n) push/pop** of the extreme element. For "top k",
keep a heap of **size k** of the *opposite* kind: a **min**-heap for the k **largest**.
Its top is the weakest survivor, so you evict it. That's **O(n log k)** instead of O(n log n).

## Rust toolkit

- `BinaryHeap<T>` is a **max-heap**. Wrap items in `std::cmp::Reverse` for a min-heap.
- **Tuples order left to right**: `Reverse((priority, id))`. Put the priority first.
- `BinaryHeap::from(vec)` heapifies in O(n).
- See the [BinaryHeap](../cheatsheet/heap.md) cheat sheet for custom `Ord`.

## Template: top k

```rust
{{#include ../../../src/dsa/_09_heaps.rs:kth_largest}}
```

## Template: K-way merge

```rust
{{#include ../../../src/dsa/_09_heaps.rs:merge_k_sorted}}
```

## Template: Dijkstra

```rust
{{#include ../../../src/dsa/_09_heaps.rs:dijkstra}}
```

## Rust gotchas

- **Max-heap by default.** Forgetting `Reverse` is the classic bug.
- **`f64` isn't `Ord`**, so `BinaryHeap<f64>` doesn't compile. Use integer distances (squared
  distance for "k closest points"), or a wrapper struct whose `Ord` uses `total_cmp`.
- **There's no decrease-key.** Push the new entry and skip **stale** ones when popped
  (`if d > dist[u] { continue }`).
- **Iteration order isn't sorted.** `heap.iter()` / `into_iter()` is arbitrary. Use
  `into_sorted_vec()` or pop in a loop.
- **`u64::MAX` as infinity** + a weight overflows. Only add after confirming the node was reached.

## Practice

- [ ] [Kth Largest Element in an Array (215)](https://leetcode.com/problems/kth-largest-element-in-an-array/)
- [ ] [Last Stone Weight (1046)](https://leetcode.com/problems/last-stone-weight/)
- [ ] [K Closest Points to Origin (973)](https://leetcode.com/problems/k-closest-points-to-origin/)
- [ ] [Top K Frequent Elements (347)](https://leetcode.com/problems/top-k-frequent-elements/)
- [ ] [Merge k Sorted Lists (23)](https://leetcode.com/problems/merge-k-sorted-lists/)
- [ ] [Task Scheduler (621)](https://leetcode.com/problems/task-scheduler/)
- [ ] [Find Median from Data Stream (295)](https://leetcode.com/problems/find-median-from-data-stream/)
