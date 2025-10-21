pub fn sort_valley_shaped_array(arr: Vec<i32>) -> Vec<i32> {
    // sort function here

    let n = arr.len();
    println!("the length is: {n}");
    let (mut left, mut right): (usize, usize) = (0, n - 1);
    let mut target_index: usize = n - 1;
    let mut result: Vec<i32> = Vec::new();
    println!("the target_index is: {target_index}");
    println!("the left is: {left}");
    println!("the right is: {right}");

    if arr.is_empty() {
        return vec![];
    }

    'main_loop: while left < right {
        let left_value: i32;
        let right_value: i32;

        if let Some(lv) = arr.get(left) {
            left_value = *lv
        } else {
            println!("left value is out of bounds");
            break 'main_loop;
        }

        if let Some(rv) = arr.get(right) {
            right_value = *rv
        } else {
            println!("right value is out of bounds");
            break 'main_loop;
        }
        println!("in loop value: left_value: {left_value}");

        println!("in loop value: right_value: {right_value}");

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
            right += 1;
        }
    }

    result
}

pub fn test_sort_valley_shaped_array() {
    println!("Now we will test the valley shaped array sorter");

    let arr = vec![10, 9, 8, 7, 1, 2, 3, 4];
    let result = sort_valley_shaped_array(arr.clone());
    println!("Test data: {arr:?}");
    println!("Result Data: {result:?}");
}
