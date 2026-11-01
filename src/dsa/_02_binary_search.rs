//! ## Triggers
//! Sorted data, need to find some data inside, 0(n log n)
//!
//! ## Keywords
//! | dummy | dummy | dummy | dummy | dummy |
//! |---|---|---|---|---|
//!
//! ```
//! pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
//!     let (mut left, mut right) = (0, nums.len() - 1);
//!
//!     // base cases
//!     if target < nums[0] || target > nums[right] {
//!         return None;
//!     } else if nums[0] == target {
//!         return Some(left);
//!   } else if nums[right] == target {
//!       return Some(right);
//!   }
//!
//!   while left < right - 1 {
//!       let middle_index = (left + right) / 2;
//!       let middle_value = nums[middle_index];
//!
//!       if nums[middle_index] == target {
//!           return Some(middle_index);
//!       } else if middle_value > target {
//!           right = middle_index;
//!       } else {
//!           left = middle_index
//!       }
//!    }
//!    None
//!}
//! ```

pub fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let (mut left, mut right) = (0, nums.len() - 1);

    // base cases
    if target < nums[0] || target > nums[right] {
        return None;
    } else if nums[0] == target {
        return Some(left);
    } else if nums[right] == target {
        return Some(right);
    }

    while left < right - 1 {
        let middle_index = (left + right) / 2;
        let middle_value = nums[middle_index];

        if nums[middle_index] == target {
            return Some(middle_index);
        } else if middle_value > target {
            right = middle_index;
        } else {
            left = middle_index
        }
    }
    None
}

pub fn test_binary_search() {
    let nums_1 = [0, 1, 5, 8, 10, 13, 25, 78];
    let target_1 = 8;

    let nums_2 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target_2 = 10;

    let nums_3 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target_3 = 20;

    let result_1 = binary_search(&nums_1, target_1);
    let result_2 = binary_search(&nums_2, target_2);
    let result_3 = binary_search(&nums_3, target_3);

    // println!("");

    assert_eq!(binary_search(&nums_1, target_1), Some(3));
    assert_eq!(binary_search(&nums_2, target_2), Some(10));
    assert_eq!(binary_search(&nums_3, target_3), None);
}
