//! BinaryHeap<T>: a MAX-heap. push/pop O(log n), peek O(1).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[test]
fn max_heap() {
    // ANCHOR: max_heap
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(9);
    heap.push(1);
    let top = heap.peek().copied(); // Some(9)  the LARGEST
    let popped = heap.pop(); //        Some(9)
    let heap2: BinaryHeap<i32> = vec![4, 8, 2].into(); // heapify in O(n)
    let sorted = heap2.into_sorted_vec(); // [2, 4, 8] ascending
    // ANCHOR_END: max_heap
    assert_eq!((top, popped, heap.len()), (Some(9), Some(9), 2));
    assert_eq!(sorted, [2, 4, 8]);
}

#[test]
fn min_heap() {
    // ANCHOR: min_heap
    // Wrap values in Reverse to flip the ordering into a MIN-heap.
    let mut heap = BinaryHeap::new();
    for x in [5, 1, 8] {
        heap.push(Reverse(x));
    }
    let Reverse(smallest) = heap.pop().unwrap(); // 1
    // Tuples compare left to right, so the priority goes first:
    let mut pq: BinaryHeap<Reverse<(u32, usize)>> = BinaryHeap::new(); // (dist, node)
    pq.push(Reverse((7, 0)));
    pq.push(Reverse((2, 3)));
    let Some(Reverse((dist, node))) = pq.pop() else { panic!() }; // (2, 3)
    // ANCHOR_END: min_heap
    assert_eq!(smallest, 1);
    assert_eq!((dist, node), (2, 3));
}

#[test]
fn top_k() {
    // ANCHOR: top_k
    // k largest: keep a MIN-heap of size k. O(n log k)
    fn k_largest(nums: &[i32], k: usize) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for &x in nums {
            heap.push(Reverse(x));
            if heap.len() > k {
                heap.pop(); // evict the smallest
            }
        }
        heap.into_iter().map(|Reverse(x)| x).collect()
    }
    // ANCHOR_END: top_k
    let mut got = k_largest(&[3, 1, 5, 12, 2, 11], 3);
    got.sort();
    assert_eq!(got, [5, 11, 12]);
}

#[test]
fn custom_ord() {
    // ANCHOR: custom_ord
    // A custom priority: implement Ord (and the three traits it needs).
    #[derive(PartialEq, Eq, Debug)]
    struct Task {
        priority: u8,
        name: &'static str,
    }
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.priority.cmp(&other.priority).then_with(|| other.name.cmp(self.name))
        }
    }
    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut heap = BinaryHeap::from([Task { priority: 1, name: "b" }, Task { priority: 5, name: "a" }]);
    // f64 isn't Ord. Store integers, or wrap it in a struct whose Ord uses f64::total_cmp.
    // ANCHOR_END: custom_ord
    assert_eq!(heap.pop().unwrap().name, "a");
}
