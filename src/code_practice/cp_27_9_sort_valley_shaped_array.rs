pub fn sort_valley_shaped_array(arr: Vec<i32>) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let n = arr.len();
    let (mut left, mut right): (usize, usize) = (0, n - 1);
    let mut target_index: usize = n - 1;
    let mut result: Vec<i32> = vec![0; n];

    'main_loop: while left < right {
        let left_value: i32 = arr[left];
        let right_value: i32 = arr[right];

        if left == right {
            result[target_index] = left_value;
            break 'main_loop;
        } else if left_value > right_value {
            result[target_index] = left_value;
            target_index -= 1;
            left += 1;
        } else {
            result[target_index] = right_value;
            target_index -= 1;
            right -= 1;
        }
    }

    result
}

pub fn test_sort_valley_shaped_array() {
    println!("Now we will test the valley shaped array sorter");

    let arr = vec![10, 5, 3, 8];
    let result = sort_valley_shaped_array(arr.clone());
    println!("Test data: {arr:?}");
    println!("Result Data: {result:?}");

    let arr = vec![10, 9, 8, 3, 5, 7, 20];
    let result = sort_valley_shaped_array(arr.clone());
    println!("Test data: {arr:?}");
    println!("Result Data: {result:?}");

    let arr = vec![];
    let result = sort_valley_shaped_array(arr.clone());
    println!("Test data: {arr:?}");
    println!("Result Data: {result:?}");

    let arr = vec![10, 3, 8];
    let result = sort_valley_shaped_array(arr.clone());
    println!("Test data: {arr:?}");
    println!("Result Data: {result:?}");
}
