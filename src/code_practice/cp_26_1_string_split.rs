pub fn string_split(s: String, c: char) -> Vec<String> {
    let mut result_array: Vec<String> = Vec::new();
    let s_iterator = s.char_indices();
    let mut left_i: usize = 0;
    let n = s.len();

    for (i, char) in s_iterator {
        // if is last char
        if i == n - 1 {
            if char != c {
                let string_slice: String = s[left_i..(i + 1)].to_string();
                result_array.push(string_slice);
            } else {
                let string_slice: String = s[left_i..i].to_string();
                result_array.push(string_slice);
            }
        }
        // else if we're not on last char
        else if char == c {
            if i > left_i {
                let string_slice: String = s[left_i..i].to_string();
                result_array.push(string_slice);
                left_i = i + 1;
            } else {
                left_i = i + 1;
            }
        }
    }

    result_array
}

pub fn test_string_split() {
    println!("Starting tests on string_split()");

    let testing_string = "This is a test string".to_string();
    let string_split_result = string_split(testing_string.clone(), ' ');
    println!("input is: {testing_string}");
    println!("result: {string_split_result:?}");

    let testing_string = "Here    is   --- another one   !".to_string();
    let string_split_result = string_split(testing_string.clone(), ' ');
    println!("input is: {testing_string}");
    println!("result: {string_split_result:?}");

    let testing_string = "  and something   more challenging !".to_string();
    let string_split_result = string_split(testing_string.clone(), ' ');
    println!("input is: {testing_string}");
    println!("result: {string_split_result:?}");
}
