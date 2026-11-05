//! Stacks (Vec) and queues/deques (VecDeque).

use std::collections::VecDeque;

#[test]
fn stack() {
    // ANCHOR: stack
    // A Vec IS your stack. push and pop both work on the end, O(1).
    let mut stack: Vec<char> = Vec::new();
    stack.push('(');
    stack.push('[');
    let top = stack.last().copied(); // peek: Some('[')  (.copied() so we can pop next)
    let popped = stack.pop(); //        Some('[')
    let empty = stack.is_empty();
    // ANCHOR_END: stack
    assert_eq!((top, popped, empty), (Some('['), Some('['), false));
}

#[test]
fn queue() {
    // ANCHOR: queue
    // VecDeque is a ring buffer: O(1) push/pop at BOTH ends.
    // Never use Vec::remove(0) as a queue. That's O(n).
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(1); //                 enqueue
    q.push_back(2);
    q.push_front(0); //                deque: add to the front
    let front = q.front().copied(); // Some(0) peek
    let back = q.back().copied(); //   Some(2)
    let next = q.pop_front(); //       Some(0)  dequeue
    let last = q.pop_back(); //        Some(2)
    let by_idx = q[0]; //              1  (indexable!)
    let from: VecDeque<i32> = vec![1, 2, 3].into();
    let seeded = VecDeque::from([(0, 0)]); // BFS start: queue with one item
    // ANCHOR_END: queue
    assert_eq!((front, back, next, last, by_idx), (Some(0), Some(2), Some(0), Some(2), 1));
    assert_eq!((from.len(), seeded.len()), (3, 1));
}

#[test]
fn deque_extras() {
    // ANCHOR: deque_extras
    let mut d: VecDeque<i32> = (1..=5).collect();
    d.rotate_left(2); //                        [3, 4, 5, 1, 2]
    let contiguous = d.make_contiguous(); //    &mut [i32], so slice methods work
    contiguous.sort();
    let v: Vec<i32> = d.into_iter().collect(); // [1, 2, 3, 4, 5]
    // ANCHOR_END: deque_extras
    assert_eq!(v, [1, 2, 3, 4, 5]);
}
