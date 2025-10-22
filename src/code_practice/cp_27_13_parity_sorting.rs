pub fn parity_sorting(arr: Vec<i32>) -> Vec<i32> {
    let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);
    let mut arr = arr.clone();

    while left < right {
        if arr[left] % 2 == 0 {
            left += 1
        } else if arr[right] % 2 == 1 {
            right -= 1
        } else {
            (arr[left], arr[right]) = (arr[right], arr[left]);
            left += 1;
            right -= 1;
        }
    }
    arr
}

pub fn test_parity_sorting() {
    println!("This function sorts arrays so that all even numbers are sorted before even numbers");
    let arr: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let test_result = parity_sorting(arr.clone());
    println!("Test Data: {arr:?}");
    println!("Result: {test_result:?}");
}
