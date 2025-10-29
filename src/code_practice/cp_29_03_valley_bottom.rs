// return the lowest number in an array with a prefix that is sorted negatively, and a suffix
// sorted positively. no repeating values.
pub fn valley_bottom(arr: Vec<i32>) -> i32 {
    let (mut left, mut right): (usize, usize) = (0, arr.len() - 1);

    // while left and right aren't next to eachother
    while right > left + 1 {
        let mid = (left + right) / 2;
        println!("left: {left}, right: {right}, mid: {mid}");
        if arr[mid] > arr[mid + 1] {
            left = mid;
        } else {
            right = mid;
        }
    }
    return arr[right];
}

pub fn test_valley_bottom() {
    // test here
    println!("Testing the Valley Bottom");

    let arr = vec![10, 8, 6, 4, 2, 0, 1, 3, 5, 7, 9, 11, 13];
    let result = valley_bottom(arr.clone());
    println!("Test Array: {arr:?}");
    println!("Result: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_sorted_array_1() {
        // test here
        let arr = vec![10, 8, 6, 4, 2, 0, 1, 3, 5, 7, 9, 11, 13];
        let result = valley_bottom(arr);

        assert!(result == 0);
    }

    #[test]
    fn test_search_sorted_array_2() {
        // test here
        let arr = vec![10, -100, 8];
        let result = valley_bottom(arr);

        assert!(result == -100);
    }

    #[test]
    fn test_search_sorted_array_3() {
        // test here
        let arr = vec![100, 50, 51, 200];
        let result = valley_bottom(arr);

        assert!(result == 50);
    }
}
