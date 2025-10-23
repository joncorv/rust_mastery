// given an array of integers, and a pivot value. ensure that all values to the right of the pivot
// are greater than pivot value, and all to the left are less than the pivot value

pub fn quicksort_partition(arr: Vec<i32>, pivot: i32) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);
    let mut arr: Vec<i32> = arr.clone();

    while left < right {
        if arr[left] <= pivot {
            left += 1;
        } else if arr[right] > pivot {
            right -= 1;
        } else {
            (arr[left], arr[right]) = (arr[right], arr[left]);
            left += 1;
            right -= 1;
        }
    }

    left = 0;
    right = arr.len() - 1;
    while left < right {
        if arr[left] != pivot {
            left += 1;
        } else if arr[right] >= pivot {
            right -= 1;
        } else {
            (arr[left], arr[right]) = (arr[right], arr[left]);
            left += 1;
            right -= 1;
        }
    }

    arr
}

pub fn test_quicksort_partition() {
    println!("testing quicksort partition. All numbers > pivot ->. All numbers < Pivot <-");
    let arr = vec![0, 5, 10, 5, 9, 3];
    let pivot: i32 = 5;
    let test_result = quicksort_partition(arr.clone(), pivot.clone());
    println!("test array: {arr:?}, test pivot: {pivot}");
    println!("result: {test_result:?}");

    let arr = vec![-10, 5, 20, 4, 17, 3, 85, 12, 13, 5, 7, 9];
    let pivot: i32 = 12;
    let test_result = quicksort_partition(arr.clone(), pivot.clone());
    println!("test array: {arr:?}, test pivot: {pivot}");
    println!("result: {test_result:?}");
}
