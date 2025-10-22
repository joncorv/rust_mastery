// here we need to merge 2 sorted arrays into a single sorted array.

pub fn two_sum(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let (mut index_a, mut index_b): (usize, usize) = (0, 0);
    let (mut val_a, mut val_b): (i32, i32) = (0, 0);
    let (mut valid_a, mut valid_b): (bool, bool) = (true, true);
    let mut result: Vec<i32> = Vec::new();

    while valid_a && valid_b {
        if let Some(va) = a.get(index_a) {
            val_a = *va;
        } else {
            valid_a = false;
        }

        if let Some(vb) = b.get(index_b) {
            val_b = *vb;
        } else {
            valid_b = false;
        }

        if val_a == val_b {
            result.push(val_a);
            result.push(val_b);
            index_a += 1;
            index_b += 1;
        } else if val_a < val_b {
            result.push(val_a);
            index_a += 1;
        } else {
            result.push(val_b);
            index_b += 1;
        }
    }

    // here we need to also push the leftover values of the larger array into result
    while index_a < a.len() {
        result.push(a[index_a]);
        index_a += 1;
    }

    while index_b < b.len() {
        result.push(b[index_b]);
        index_b += 1;
    }

    result
}

pub fn test_two_sum() {
    // test here

    println!("Here we begin testing our integer array merge function");

    let a = vec![0, 1, 2, 3, 4, 5, 6, 20, 21, 22, 23];
    let b = vec![0, 5, 6, 20, 30, 40];
    let test_result = two_sum(a.clone(), b.clone());
    println!("Tests: {a:?}, {b:?}");
    println!("Result: {test_result:?}");

    let a = vec![0, 30, 38, 59, 100];
    let b = vec![5, 10, 15, 20, 30];
    let test_result = two_sum(a.clone(), b.clone());
    println!("Tests: {a:?}, {b:?}");
    println!("Result: {test_result:?}")
}
