// input data is "hello my name is jon."

pub fn string_split(s: String, c: char) -> Vec<String> {
    let mut result_array: Vec<String> = Vec::new();
    // let s_iterator: (usize, Vec<char>) = s.char_indices();
    let s_iterator = s.char_indices();
    let mut left_i: usize = 0;
    let n = s.len();

    for (i, char) in s_iterator {
        if char == c {
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

// run a for loop of the string w/ enumeration
// if char matches c
// if i > left_i
// append a string slice to result array start -> i - 1
// left = i +1
// else = i+1
//
