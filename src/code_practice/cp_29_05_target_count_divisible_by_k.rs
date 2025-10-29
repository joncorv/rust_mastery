// given a sorted array of integers, find if the total number of target integers is divisble by k

pub fn target_count_divisible_by_k(arr: Vec<i32>, target: i32, k: i32) -> bool {
    // if the target exists
    if target_exists(&arr, target) {
        let first = find_first_occurance(&arr, target);
        let last = find_last_occurance(&arr, target);

        if (last as i32 - first as i32) % k == 0 {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn target_exists(arr: &Vec<i32>, target: i32) -> bool {
    // classic binary search
    let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);

    if arr[left] == target || arr[right] == target {
        return true;
    } else {
        while left < right {
            let mid = (left + right) / 2;
            println!("left: {left}, right: {right}, mid: {mid}");
            if arr[mid] == target {
                return true;
            } else if arr[mid] < target {
                right = mid;
            } else {
                left = mid;
            }
        }
    }
    return false;
}

fn find_first_occurance(arr: &Vec<i32>, target: i32) -> usize {
    let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);

    if arr[left] == target {
        return left;
    }

    while right > left + 1 {
        let mid = (left + right) / 2;

        if arr[mid] >= target {
            right = mid
        } else {
            left = mid
        }
    }

    return right;
}

fn find_last_occurance(arr: &Vec<i32>, target: i32) -> usize {
    let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);

    if arr[right] == target {
        return right;
    }

    while right > left + 1 {
        let mid = (left + right) / 2;

        if arr[mid] <= target {
            left = mid
        } else {
            right = mid
        }
    }

    return left;
}

pub fn test_target_count_divisible_by_k() {
    println!("testing target_count_divisible_by_k");
    let arr = vec![-10, -5, 0, 1, 2, 2, 2, 2, 2, 4, 7, 9];
    let target = 2;
    let k = 5;
    let test_result = target_count_divisible_by_k(arr.clone(), target, k);
    println!("test arr: {arr:?}, k: {k}");
    println!("result: {test_result:?}");
}
