//! ## Triggers
//! The input contains one or more of Array, Strings, or LinkedLists. We need to use 0(1) memory
//! to modify the input in place. The input is sorted. The naive solution is 0(n²) and we want to
//! get to 0(n)
//!
//! ## Keywords
//! | palindrome | reverse | swap | merge | partition |
//! |---|---|---|---|---|
//!
//! ## Movement Patterns
//! - Inward Pointers
//! - Slow and Fast pointers
//! - Paralell pointers in two arrays

/// ## Classic two-pointer problem
/// pointers move from outside.
/// need to reverse the string, and need to do it in plae
/// <https://leetcode.com/problems/reverse-string/>
///
/// ```
/// pub fn reverse_string(s: &mut [char]) {
///     let (mut left, mut right) = (0, s.len() - 1);
///
///     while left < right {
///         (s[left], s[right]) = (s[right], s[left]);
///         left += 1;
///         right -= 1;
///     }
/// }
/// ```
///
pub fn reverse_string(s: &mut [char]) {
    let (mut left, mut right) = (0, s.len() - 1);

    while left < right {
        (s[left], s[right]) = (s[right], s[left]);
        left += 1;
        right -= 1;
    }
}
#[test]
pub fn test_reverse_string() {
    let mut test_1 = vec!['a', 'b', 'c', 'd'];
    let mut test_2 = vec!['1', '2', '3', '4', '5'];

    reverse_string(&mut test_1);
    reverse_string(&mut test_2);

    assert_eq!(test_1, ['d', 'c', 'b', 'a']);
    assert_eq!(test_2, ['5', '4', '3', '2', '1']);
}

/// ## Classic two-pointer problem. Slow/Fast
/// 283. Move Zeroes
/// <https://leetcode.com/problems/move-zeroes/>
/// ```
/// pub fn move_zeroes(nums: &mut [i32]) {
///     // body here
///     let (mut left, mut right) = (0, 1);
///
///     while right < nums.len() {
///         // left zero, right not zero
///         if nums[left] == 0 && nums[right] != 0 {
///             (nums[left], nums[right]) = (nums[right], nums[left]);
///             right += 1;
///             left += 1;
///
///             // both are zero
///         } else if nums[left] == 0 && nums[right] == 0 {
///             right += 1;
///
///             // both are not zero
///             // left is not zero, right is zero
///         } else {
///             right += 1;
///             left += 1;
///         }
///     }
/// }
/// ```
pub fn move_zeroes(nums: &mut [i32]) {
    // body here
    let (mut left, mut right) = (0, 1);

    while right < nums.len() {
        // left zero, right not zero
        if nums[left] == 0 && nums[right] != 0 {
            (nums[left], nums[right]) = (nums[right], nums[left]);
            right += 1;
            left += 1;

            // both are zero
        } else if nums[left] == 0 && nums[right] == 0 {
            right += 1;

            // both are not zero
            // left is not zero, right is zero
        } else {
            right += 1;
            left += 1;
        }
    }
}
#[test]
pub fn test_move_zeroes() {
    let mut test_1 = vec![0, 1, 0, 3, 12];
    let mut test_2 = vec![1, 0, 2, 3, 0, 5, 26, 0, 100, 0];

    move_zeroes(&mut test_1);
    move_zeroes(&mut test_2);

    println!("test1 = {test_1:?}");
    println!("test2 = {test_2:?}");

    assert_eq!(test_1, [1, 3, 12, 0, 0]);
    assert_eq!(test_2, [1, 2, 3, 5, 26, 100, 0, 0, 0, 0]);
}
