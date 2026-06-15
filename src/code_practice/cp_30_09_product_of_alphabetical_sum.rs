use std::collections::HashSet;

// given an array of strings, where a = 1, b = 2, etc
// get the sum of each of those strings.
// then we need to find if any triplets of those string sums when multiplied by eachother equals a
// target sum

pub fn product_of_alphabetical_sums(arr: Vec<String>, target: i32) -> bool {
    let mut string_sums: Vec<i32> = Vec::new();
    let mut hash_sums: HashSet<i32> = HashSet::new();

    for word in arr {
        let mut sum: i32 = 0;
        for c in word.chars() {
            sum += (c as i32) - 96;
        }
        string_sums.push(sum);
        hash_sums.insert(sum);
    }
    let mut left: usize;
    let mut right: usize;
    let num_sums = string_sums.len();
    left = 0;

    while left < num_sums {
        right = left;
        while right < num_sums {
            let search_target = (string_sums[left] * string_sums[right]) / target;
            if hash_sums.contains(&search_target) {
                return true;
            } else {
                right += 1;
            }
        }
        left += 1;
    }
    false
}

pub fn test_product_of_alphabetical_sums() {
    let arr = vec!["abc".to_string(), "def".to_string(), "hig".to_string()];
    let target: i32 = 24;
    let test_results = product_of_alphabetical_sums(arr.clone(), target.clone());
    println!("test data: {arr:?}");
    println!("results: {test_results:?}");
}
