pub fn interval_intersection(arr1: Vec<Vec<i32>>, arr2: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if arr1.is_empty() || arr2.is_empty() {
        return result;
    }

    let min_val: i32 = arr1[0][0].max(arr2[0][0]);
    let max_val: i32;

    {
        // get the smallest last value
        let arr1_last_index = arr1.len() - 1;
        let arr2_last_index = arr2.len() - 1;
        max_val = arr1[arr1_last_index][1].min(arr2[arr2_last_index][1]);
    }

    println!("min: {min_val}, max: {max_val}");

    let (mut arr1_index, mut arr2_index): (usize, usize) = (0, 0);

    for search_val in min_val..=max_val {
        println!("searching for: {search_val}");
        println!("arr1_index: {arr1_index}, arr2_index: {arr2_index}");
        let (mut valid1, mut valid2): (bool, bool) = (false, false);

        let arr1_min_boundary = arr1[arr1_index][0];
        let arr1_max_boundary = arr1[arr1_index][1];
        println!("current arr1 min: {arr1_min_boundary}, max: {arr1_max_boundary}");
        if search_val >= arr1_min_boundary && search_val <= arr1_max_boundary {
            valid1 = true
        } else if search_val == arr1_max_boundary {
            arr1_index += 1;
        }

        println!("current arr2 max:{}", arr2[arr2_index][1]);
        if search_val >= arr2[arr2_index][0] && search_val <= arr2[arr2_index][1] {
            valid2 = true
        } else if search_val >= arr2[arr2_index][1] {
            arr2_index += 1;
        }

        if valid1 && valid2 {
            result.push(search_val);
        }
    }

    return result;
}

pub fn test_interval_intersection() {
    // test here
    println!("Here we are testing for intersections");
    let arr1 = vec![vec![0, 4], vec![5, 10], vec![15, 20]];
    let arr2 = vec![vec![0, 3], vec![5, 10], vec![15, 20]];
    let test_result = interval_intersection(arr1.clone(), arr2.clone());
    println!("Test Data1: {arr1:?}");
    println!("Test Data2: {arr2:?}");
    println!("Test Result: {test_result:?}");
}
