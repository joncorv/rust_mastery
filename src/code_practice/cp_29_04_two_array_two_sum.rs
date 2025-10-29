pub fn two_array_two_sum(sorted_arr: Vec<i32>, unsorted_arr: Vec<i32>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();

    for (index, n) in unsorted_arr.iter().enumerate() {
        let target = n * -1;

        if let Some(i) = binary_search(&sorted_arr, target) {
            result.push(vec![i, index]);
        }
    }

    return result;
}

fn binary_search(sorted_arr: &Vec<i32>, target: i32) -> Option<usize> {
    let (mut left, mut right): (usize, usize) = (0, sorted_arr.len() - 1);

    if sorted_arr[left] == target {
        return Some(left);
    } else if sorted_arr[right] == target {
        return Some(right);
    }

    while left < right {
        let mid = (left + right) / 2;
        if sorted_arr[mid] == target {
            return Some(mid);
        } else if sorted_arr[mid] < target {
            left = mid;
        } else {
            right = mid;
        }
    }
    return None;
}

pub fn test_two_array_two_sum() {
    println!("Testing two_array_two_sum");

    let sorted_arr = vec![0, 1, 2, 3, 4];
    let unsorted_arr = vec![0, 3, -2, -4];
    let test_result = two_array_two_sum(sorted_arr.clone(), unsorted_arr.clone());
    println!("test sort: {sorted_arr:?}, unsort: {unsorted_arr:?}");
    println!("result: {test_result:?}");
}
