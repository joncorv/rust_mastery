#![allow(dead_code, unused_imports, unused_variables)]

pub fn valid_palindrome(s: String) -> bool {
    let mut start: usize = 0;
    let mut end: usize = s.len() - 1;
    let s: Vec<char> = s.chars().collect();

    while start < end {
        if s[start] != s[end] {
            return false;
        } else {
            start += 1;
            end -= 1;
        }
    }

    true
}

pub fn test_valid_palindrome() {
    println!("we are testing valid palindromes");

    let test_string = "racecar";
    println!("Testing: {test_string}. This should return true");
    let test = valid_palindrome(test_string.to_string());
    println!("Result: {test:?}");

    let test_string = "ra cecar";
    println!("Testing: {test_string}. This should return false");
    let test = valid_palindrome(test_string.to_string());
    println!("Result: {test:?}");

    let test_string = "abcdcba";
    println!("Testing: {test_string}. This should return true");
    let test = valid_palindrome(test_string.to_string());
    println!("Result: {test:?}");
}
