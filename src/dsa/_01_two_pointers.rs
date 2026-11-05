//! Two Pointers. Notes live in book/src/patterns/two_pointers.md; code is included by ANCHOR.

// ANCHOR: reverse_string
/// 344. Reverse String: inward pointers. <https://leetcode.com/problems/reverse-string/>
pub fn reverse_string(s: &mut [char]) {
    if s.is_empty() {
        return; // s.len() - 1 would underflow
    }
    let (mut left, mut right) = (0, s.len() - 1);

    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}
// ANCHOR_END: reverse_string

#[test]
pub fn test_reverse_string() {
    let mut test_1 = vec!['a', 'b', 'c', 'd'];
    let mut test_2 = vec!['1', '2', '3', '4', '5'];
    let mut empty: Vec<char> = vec![];

    reverse_string(&mut test_1);
    reverse_string(&mut test_2);
    reverse_string(&mut empty);

    assert_eq!(test_1, ['d', 'c', 'b', 'a']);
    assert_eq!(test_2, ['5', '4', '3', '2', '1']);
    assert!(empty.is_empty());
}

// ANCHOR: move_zeroes
/// 283. Move Zeroes: my first solution, slow/fast pointers.
/// <https://leetcode.com/problems/move-zeroes/>
pub fn move_zeroes(nums: &mut [i32]) {
    let (mut left, mut right) = (0, 1);

    while right < nums.len() {
        // left zero, right not zero
        if nums[left] == 0 && nums[right] != 0 {
            nums.swap(left, right);
            right += 1;
            left += 1;

            // both are zero
        } else if nums[left] == 0 && nums[right] == 0 {
            right += 1;

            // left is not zero (right may or may not be)
        } else {
            right += 1;
            left += 1;
        }
    }
}
// ANCHOR_END: move_zeroes

// ANCHOR: move_zeroes_write_ptr
/// 283. Move Zeroes: the "write pointer" version. `write` is the next slot for a
/// non-zero; `read` scans everything. Same O(n) / O(1), only one case to explain.
pub fn move_zeroes_write_ptr(nums: &mut [i32]) {
    let mut write = 0;
    for read in 0..nums.len() {
        if nums[read] != 0 {
            nums.swap(write, read);
            write += 1;
        }
    }
}
// ANCHOR_END: move_zeroes_write_ptr

#[test]
pub fn test_move_zeroes() {
    for f in [move_zeroes, move_zeroes_write_ptr] {
        let mut test_1 = vec![0, 1, 0, 3, 12];
        let mut test_2 = vec![1, 0, 2, 3, 0, 5, 26, 0, 100, 0];
        let mut empty: Vec<i32> = vec![];

        f(&mut test_1);
        f(&mut test_2);
        f(&mut empty);

        assert_eq!(test_1, [1, 3, 12, 0, 0]);
        assert_eq!(test_2, [1, 2, 3, 5, 26, 100, 0, 0, 0, 0]);
    }
}
