pub fn target_count_divisible_by_k(arr: Vec<i32>, target: i32, k: i32) -> bool {
    return false;
}

fn find_first_occurance(arr: &Vec<i32>, target: i32) -> Option<usize> {
    None
}

fn find_last_occurance() {}

pub fn test_target_count_divisible_by_k() {
    println!("testing target_count_divisible_by_k");
    let arr = vec![-10, -5, 0, 1, 2, 2, 2, 2, 2, 4, 7, 9];
    let target = 2;
    let k = 5;
    let test_result = target_count_divisible_by_k(arr.clone(), target, k);
    println!("test arr: {arr:?}, k: {k}");
    println!("result: {test_result:?}");
}
