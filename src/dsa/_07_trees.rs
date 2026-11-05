//! Trees. Notes live in book/src/patterns/trees.md.
//! TreeNode itself is defined in src/ds/pointers.rs.

use crate::ds::pointers::Tree;
use std::collections::VecDeque;
use std::rc::Rc;

// ANCHOR: max_depth
/// 104. Maximum Depth: the basic DFS shape. Handle None, recurse on children, combine.
pub fn max_depth(root: &Tree) -> i32 {
    let Some(node) = root else { return 0 };
    let n = node.borrow();
    1 + max_depth(&n.left).max(max_depth(&n.right))
}
// ANCHOR_END: max_depth

// ANCHOR: level_order
/// 102. Level Order Traversal: BFS, one level per outer loop iteration.
pub fn level_order(root: &Tree) -> Vec<Vec<i32>> {
    let mut out = vec![];
    let mut q = VecDeque::new();
    if let Some(r) = root {
        q.push_back(Rc::clone(r));
    }
    while !q.is_empty() {
        let mut level = Vec::with_capacity(q.len());
        for _ in 0..q.len() {
            // range is computed once, so pushes below don't affect it
            let node = q.pop_front().unwrap();
            let n = node.borrow();
            level.push(n.val);
            if let Some(l) = &n.left {
                q.push_back(Rc::clone(l));
            }
            if let Some(r) = &n.right {
                q.push_back(Rc::clone(r));
            }
        }
        out.push(level);
    }
    out
}
// ANCHOR_END: level_order

// ANCHOR: is_valid_bst
/// 98. Validate BST: pass the allowed (lo, hi) range DOWN the tree.
pub fn is_valid_bst(root: &Tree) -> bool {
    fn go(node: &Tree, lo: Option<i32>, hi: Option<i32>) -> bool {
        let Some(n) = node else { return true };
        let n = n.borrow();
        if lo.is_some_and(|lo| n.val <= lo) || hi.is_some_and(|hi| n.val >= hi) {
            return false;
        }
        go(&n.left, lo, Some(n.val)) && go(&n.right, Some(n.val), hi)
    }
    go(root, None, None)
}
// ANCHOR_END: is_valid_bst

// ANCHOR: invert_tree
/// 226. Invert Binary Tree: mutation through borrow_mut() + std::mem::swap.
pub fn invert_tree(root: Tree) -> Tree {
    if let Some(node) = &root {
        let mut n = node.borrow_mut();
        let n = &mut *n; // reborrow so we can touch two fields at once
        std::mem::swap(&mut n.left, &mut n.right);
        invert_tree(n.left.clone()); // clone of an Option<Rc> = refcount bump
        invert_tree(n.right.clone());
    }
    root
}
// ANCHOR_END: invert_tree

#[test]
fn test_trees() {
    use crate::ds::pointers::{leaf, node};
    let t = node(3, leaf(9), node(20, leaf(15), leaf(7)));
    assert_eq!(max_depth(&t), 3);
    assert_eq!(max_depth(&None), 0);
    assert_eq!(level_order(&t), vec![vec![3], vec![9, 20], vec![15, 7]]);
    assert!(!is_valid_bst(&t));
    assert!(is_valid_bst(&node(5, leaf(3), node(8, leaf(6), leaf(9)))));
    assert!(!is_valid_bst(&node(5, leaf(3), node(8, leaf(4), leaf(9))))); // 4 < 5 deep down
    let inv = invert_tree(t);
    assert_eq!(level_order(&inv), vec![vec![3], vec![20, 9], vec![7, 15]]);
}
