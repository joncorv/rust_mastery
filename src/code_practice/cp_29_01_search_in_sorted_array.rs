pub fn search_in_sorted_array(arr: Vec<i32>, target: i32) -> usize {
    if arr.is_empty() {
        return 0 as usize;
    }

    let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);

    while left <= right {
        if arr[left] == target {
            return left;
        } else if arr[right] == target {
            return right;
        }

        let mid = (left + right) / 2;
        if arr[mid] == target {
            return mid;
        } else if arr[mid] < target {
            left = mid;
        } else {
            right = mid;
        }
    }

    return 0 as usize;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_sorted_array_1() {
        let arr: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let target: i32 = 3;
        let result = search_in_sorted_array(arr, target);
        assert!(result == 3);
    }

    #[test]
    fn test_search_sorted_array_empty() {
        let arr: Vec<i32> = Vec::new();
        let target: i32 = 3;
        let result = search_in_sorted_array(arr, target);
        assert!(result == 0);
    }

    #[test]
    fn test_search_sorted_array_2() {
        let arr: Vec<i32> = vec![-100, 1, 2, 3, 5, 200];
        let target: i32 = 5;
        let result = search_in_sorted_array(arr, target);
        assert!(result == 4);
    }
}
