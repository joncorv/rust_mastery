#![allow(dead_code, unused_imports, unused_variables)]

pub fn valid_palindrome_whitespace(s: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut start: usize = 0;
    let mut end: usize = s.len() - 1;
    let s: Vec<char> = s.chars().collect();

    while start < end {
        let (start_char, end_char);

        'start: loop {
            if let Some(val) = s.get(start) {
                if *val == ' ' {
                    start += 1;
                } else {
                    start_char = val;
                    start += 1;
                    break 'start;
                }
            } else {
                return true;
            }
        }

        'end: loop {
            if let Some(val) = s.get(end) {
                if *val == ' ' {
                    end -= 1;
                } else {
                    end_char = val;
                    end -= 1;
                    break 'end;
                }
            } else {
                return true;
            }
        }

        if start_char != end_char {
            return false;
        }
    }

    true
}

pub fn test_valid_palindrome_whitespace() {
    println!("we are testing valid palindromes");

    let test_string = "racecar";
    println!("Testing: {test_string}. This should return true");
    let test = valid_palindrome_whitespace(test_string.to_string());
    println!("Result: {test:?}");

    let test_string = "ra cecar";
    println!("Testing: {test_string}. This should return true");
    let test = valid_palindrome_whitespace(test_string.to_string());
    println!("Result: {test:?}");

    let test_string = " a bc dcba  ";
    println!("Testing: {test_string}. This should return true");
    let test = valid_palindrome_whitespace(test_string.to_string());
    println!("Result: {test:?}");

    let test_string = "";
    println!("Testing: {test_string}. This should return true");
    let test = valid_palindrome_whitespace(test_string.to_string());
    println!("Result: {test:?}");

    let test_string = "    a !   ";
    println!("Testing: {test_string}. This should return false");
    let test = valid_palindrome_whitespace(test_string.to_string());
    println!("Result: {test:?}");
}
