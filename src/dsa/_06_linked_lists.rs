//! Linked Lists. Notes live in book/src/patterns/linked_lists.md.
//! ListNode itself is defined in src/ds/pointers.rs.

use crate::ds::pointers::ListNode;

// ANCHOR: reverse_list
/// 206. Reverse Linked List: take ownership node by node.
pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take(); // detach the rest (take() leaves None behind)
        node.next = prev; //        point back
        prev = Some(node); //       advance
    }
    prev
}
// ANCHOR_END: reverse_list

// ANCHOR: middle_node
/// 876. Middle of the Linked List: slow/fast pointers over SHARED references.
pub fn middle_node(head: &Option<Box<ListNode>>) -> Option<&ListNode> {
    let mut slow = head.as_deref();
    let mut fast = head.as_deref();
    while let Some(f) = fast
        && let Some(f_next) = f.next.as_deref()
    {
        slow = slow?.next.as_deref();
        fast = f_next.next.as_deref();
    }
    slow
}
// ANCHOR_END: middle_node

// ANCHOR: merge_two
/// 21. Merge Two Sorted Lists: dummy head + a `&mut` tail cursor.
pub fn merge_two(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy;
    while let (Some(x), Some(y)) = (&a, &b) {
        let src = if x.val <= y.val { &mut a } else { &mut b };
        let mut node = src.take().unwrap();
        *src = node.next.take();
        tail = tail.next.insert(node); // insert() returns &mut to what it stored
    }
    tail.next = a.or(b); // attach whatever is left
    dummy.next
}
// ANCHOR_END: merge_two

#[test]
fn test_linked_lists() {
    use crate::ds::pointers::{list_to_vec, vec_to_list};
    assert_eq!(list_to_vec(&reverse_list(vec_to_list(&[1, 2, 3]))), [3, 2, 1]);
    assert_eq!(reverse_list(None), None);

    assert_eq!(middle_node(&vec_to_list(&[1, 2, 3, 4, 5])).map(|n| n.val), Some(3));
    assert_eq!(middle_node(&vec_to_list(&[1, 2, 3, 4])).map(|n| n.val), Some(3));
    assert_eq!(middle_node(&None).map(|n| n.val), None);

    let merged = merge_two(vec_to_list(&[1, 2, 4]), vec_to_list(&[1, 3, 4, 5]));
    assert_eq!(list_to_vec(&merged), [1, 1, 2, 3, 4, 4, 5]);
}
