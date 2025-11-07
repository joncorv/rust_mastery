use std::collections::HashSet;

// given an array of integers, return all the indices of numbers that can square the other
// for instance [i]^2 = [j]

pub fn find_all_squares(arr: Vec<i32>) -> Vec<(i32, i32)> {
    let mut result: Vec<(i32, i32)> = Vec::new();
    let mut hash_ints: HashSet<i32> = HashSet::with_capacity(arr.len());

    let arr = arr.clone();

    for n in arr {
        hash_ints.insert(n);
    }

    for n in hash_ints.clone() {
        let target = n * n;
        if let Some(_) = hash_ints.get(&target) {
            result.push((n, target));
        }
    }

    return result;
}

pub fn test_find_all_squares() {
    let arr = vec![-5, -1, 0, 1, 3, 9, 12, 27, -25];
    let test_result = find_all_squares(arr.clone());

    println!("test data: {arr:?}");
    println!("results: {test_result:?}");
}
