//! String, &str, char and bytes.

#[test]
fn basics() {
    // ANCHOR: basics
    let mut s = String::from("hello");
    s.push(' '); //                  push a char
    s.push_str("world"); //          push a &str
    let n = s.len(); //              BYTES, not chars! 11 here (ASCII)
    let chars = s.chars().count(); //  real char count, O(n)
    let is_empty = s.is_empty();
    let upper = s.to_uppercase();
    let owned: String = "slice".to_string(); // &str -> String
    let borrowed: &str = &owned; //             String -> &str (deref)
    let combined = format!("{s}!"); //          concat without moving
    // ANCHOR_END: basics
    assert_eq!((n, chars, is_empty), (11, 11, false));
    assert_eq!(upper, "HELLO WORLD");
    assert_eq!(borrowed, "slice");
    assert_eq!(combined, "hello world!");
}

#[test]
fn indexing() {
    // ANCHOR: indexing
    let s = "racecar";
    // s[0]                      // ERROR: can't index a str by integer
    let first = s.chars().next(); //            Some('r')
    let nth = s.chars().nth(3); //              Some('e')  but O(n)!
    // For index-heavy problems convert ONCE up front:
    let bytes = s.as_bytes(); //                &[u8], O(1) indexing, ASCII only
    let b = bytes[3] as char; //                'e'
    let cs: Vec<char> = s.chars().collect(); // Vec<char>, O(1) indexing, any Unicode
    let sub = &s[1..4]; //                      "ace"  byte range; panics mid-char
    // ANCHOR_END: indexing
    assert_eq!((first, nth, b, cs[3], sub), (Some('r'), Some('e'), 'e', 'e', "ace"));
}

#[test]
fn chars() {
    // ANCHOR: chars
    let c = 'b';
    let idx = (c as u8 - b'a') as usize; //       1  -> index into [0; 26]
    let back = (b'a' + 2) as char; //             'c'
    let digit = '7'.to_digit(10); //              Some(7)
    let from_digit = char::from_digit(4, 10); //  Some('4')
    let checks = (
        c.is_ascii_alphabetic(),
        c.is_ascii_digit(),
        c.is_ascii_alphanumeric(), // the palindrome-problem filter
        c.is_whitespace(),
        c.to_ascii_uppercase(),
    );
    // ANCHOR_END: chars
    assert_eq!((idx, back, digit, from_digit), (1, 'c', Some(7), Some('4')));
    assert_eq!(checks, (true, false, true, false, 'B'));
}

#[test]
fn split_and_parse() {
    // ANCHOR: split_and_parse
    let line = "  10 20   30 ";
    let words: Vec<&str> = line.split_whitespace().collect(); // ["10","20","30"]
    let nums: Vec<i32> = words.iter().map(|w| w.parse().unwrap()).collect();
    let csv: Vec<&str> = "a,b,,c".split(',').collect(); //     ["a","b","","c"]
    let (key, val) = "name=jon".split_once('=').unwrap(); //   ("name","jon")
    let trimmed = line.trim(); //                              "10 20   30"
    let num: Result<i32, _> = "abc".parse::<i32>(); //         Err(..)
    // ANCHOR_END: split_and_parse
    assert_eq!(nums, [10, 20, 30]);
    assert_eq!(csv, ["a", "b", "", "c"]);
    assert_eq!((key, val, trimmed), ("name", "jon", "10 20   30"));
    assert!(num.is_err());
}

#[test]
fn search_and_build() {
    // ANCHOR: search_and_build
    let s = "banana";
    let has = s.contains("nan"); //                 true
    let at = s.find('n'); //                        Some(2) byte index
    let starts = s.starts_with("ban"); //           true
    let rest = s.strip_prefix("ba"); //             Some("nana")
    let replaced = s.replace("a", "o"); //          "bonono"
    let rev: String = s.chars().rev().collect(); // "ananab"
    let joined = ["a", "b", "c"].join("-"); //      "a-b-c"
    let repeated = "ab".repeat(3); //               "ababab"
    // Vec<char> back to String:
    let word: String = vec!['h', 'i'].into_iter().collect();
    // ANCHOR_END: search_and_build
    assert!(has && starts);
    assert_eq!((at, rest), (Some(2), Some("nana")));
    assert_eq!((replaced.as_str(), rev.as_str()), ("bonono", "ananab"));
    assert_eq!((joined.as_str(), repeated.as_str(), word.as_str()), ("a-b-c", "ababab", "hi"));
}

#[test]
fn char_indices() {
    // ANCHOR: char_indices
    // enumerate gives char positions; char_indices gives BYTE positions (safe for slicing)
    let s = "a,bc";
    let commas: Vec<usize> = s.char_indices().filter(|&(_, c)| c == ',').map(|(i, _)| i).collect();
    // compare two strings ignoring case
    let same = "Rust".eq_ignore_ascii_case("rUST");
    // sort the letters of a word (anagram key)
    let mut key: Vec<char> = "listen".chars().collect();
    key.sort_unstable();
    let key: String = key.into_iter().collect(); // "eilnst"
    // ANCHOR_END: char_indices
    assert_eq!(commas, [1]);
    assert!(same);
    assert_eq!(key, "eilnst");
}
