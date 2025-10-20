pub fn array_intersection(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    if arr1.is_empty() || arr2.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<i32> = Vec::new();
    let (mut i1, mut i2): (usize, usize) = (0, 0);
    let (mut val1, mut val2): (i32, i32) = (0, 0);
    let (mut some1, mut some2): (bool, bool) = (true, true);

    while some1 && some2 {
        if let Some(v1) = arr1.get(i1) {
            val1 = *v1;
        } else {
            some1 = false;
        }

        if let Some(v2) = arr2.get(i2) {
            val2 = *v2;
        } else {
            some2 = false;
        }

        if val1 == val2 {
            result.push(val1);
            result.push(val2);
            i1 += 1;
            i2 += 1;
        } else if val1 < val2 {
            i1 += 1;
        } else {
            i2 += 1;
        }
    }

    return result;
}

pub fn test_array_intersection() {
    println!("We are testing for common ints among 2 sorted arrays");
    let arr1 = vec![1, 2, 5, 12, 32];
    let arr2 = vec![1, 5, 12, 24, 128];

    println!("input: {arr1:?}, and {arr2:?}");
    let test_result = array_intersection(arr1, arr2);
    println!("{test_result:?}");
}
