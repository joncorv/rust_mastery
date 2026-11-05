//! Option, Box, Rc<RefCell<..>>: what you need for LeetCode-style linked lists and trees.

use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn option_tools() {
    // ANCHOR: option_tools
    let some: Option<i32> = Some(4);
    let none: Option<i32> = None;
    let a = some.unwrap_or(0); //                  4
    let b = none.unwrap_or_default(); //           0
    let c = some.map(|x| x * 2); //                Some(8)
    let d = some.filter(|&x| x > 10); //           None
    let e = none.or(Some(1)); //                   Some(1)
    let f = some.and_then(|x| x.checked_sub(5)); // Some(-1), chain fallible steps
    let g = some.is_some_and(|x| x % 2 == 0); //   true
    let h = some.ok_or("missing"); //              Ok(4)
    let Some(val) = some else { return }; //       let-else: early exit on None
    // ANCHOR_END: option_tools
    assert_eq!((a, b, c, d, e, f, g, h, val), (4, 0, Some(8), None, Some(1), Some(-1), true, Ok(4), 4));
}

#[test]
fn option_refs() {
    // ANCHOR: option_refs
    let mut name: Option<String> = Some("jon".into());
    let r: Option<&String> = name.as_ref(); //     look without moving
    let s: Option<&str> = name.as_deref(); //      Option<String> -> Option<&str>
    let len = name.as_ref().map_or(0, |n| n.len()); // 3
    if let Some(n) = name.as_mut() {
        n.push('!'); //                            mutate in place
    }
    let taken = name.take(); //                    moves the value out, leaves None
    // ANCHOR_END: option_refs
    assert_eq!((len, taken.as_deref(), name), (3, Some("jon!"), None));
}

// ANCHOR: list_node
// LeetCode's singly linked list
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

/// Walk a list by reference.
pub fn list_to_vec(mut cur: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![];
    while let Some(node) = cur {
        out.push(node.val);
        cur = &node.next;
    }
    out
}

/// Build a list from the back: each new node owns the rest.
pub fn vec_to_list(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
// ANCHOR_END: list_node

// ANCHOR: tree_node
// LeetCode's binary tree: shared (Rc), mutable (RefCell), nullable (Option)
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
pub type Tree = Option<Rc<RefCell<TreeNode>>>;

pub fn leaf(val: i32) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left: None, right: None })))
}
pub fn node(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
// ANCHOR_END: tree_node

#[test]
fn linked_list() {
    let list = vec_to_list(&[1, 2, 3]);
    assert_eq!(list_to_vec(&list), [1, 2, 3]);
}

#[test]
fn tree() {
    // ANCHOR: tree_usage
    let root = node(1, leaf(2), node(3, leaf(4), None));
    let r = root.as_ref().unwrap(); //          &Rc<RefCell<TreeNode>>
    let left_val = r.borrow().left.as_ref().unwrap().borrow().val; // 2
    r.borrow_mut().val = 10; //                 mutate through borrow_mut()
    let shared = Rc::clone(r); //               cheap: bumps a refcount, no deep copy
    // Holding .borrow() and calling .borrow_mut() on the SAME node panics at runtime.
    // ANCHOR_END: tree_usage
    assert_eq!(left_val, 2);
    assert_eq!(shared.borrow().val, 10);
}
