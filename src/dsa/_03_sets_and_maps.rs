//! Sets & Maps. Notes live in book/src/patterns/sets_and_maps.md; code is included by ANCHOR.

use std::collections::{HashMap, HashSet};

// ANCHOR: is_anagram
/// 242. Valid Anagram: a fixed-size array as a "map" when keys are a-z.
pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts = [0i32; 26];
    for (a, b) in s.bytes().zip(t.bytes()) {
        counts[(a - b'a') as usize] += 1;
        counts[(b - b'a') as usize] -= 1;
    }
    counts.iter().all(|&c| c == 0)
}
// ANCHOR_END: is_anagram

// ANCHOR: group_anagrams
/// 49. Group Anagrams: map from a canonical KEY to the group.
pub fn group_anagrams(words: &[&str]) -> Vec<Vec<String>> {
    let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
    for w in words {
        let mut key = [0u8; 26];
        for b in w.bytes() {
            key[(b - b'a') as usize] += 1;
        }
        groups.entry(key).or_default().push(w.to_string());
    }
    groups.into_values().collect()
}
// ANCHOR_END: group_anagrams

// ANCHOR: longest_consecutive
/// 128. Longest Consecutive Sequence: O(n) by only counting up from sequence STARTS.
pub fn longest_consecutive(nums: &[i32]) -> usize {
    let set: HashSet<i32> = nums.iter().copied().collect();
    let mut best = 0;
    for &x in &set {
        if set.contains(&(x - 1)) {
            continue; // not the start of a run
        }
        let mut len = 1;
        while set.contains(&(x + len as i32)) {
            len += 1;
        }
        best = best.max(len);
    }
    best
}
// ANCHOR_END: longest_consecutive

#[test]
fn test_sets_and_maps() {
    assert!(is_anagram("anagram", "nagaram"));
    assert!(!is_anagram("rat", "car"));

    let mut groups: Vec<Vec<String>> = group_anagrams(&["eat", "tea", "tan", "ate", "nat", "bat"])
        .into_iter()
        .map(|mut g| {
            g.sort();
            g
        })
        .collect();
    groups.sort();
    assert_eq!(groups, [vec!["ate", "eat", "tea"], vec!["bat"], vec!["nat", "tan"]]);

    assert_eq!(longest_consecutive(&[100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(longest_consecutive(&[]), 0);
}

// ---------------------------------------------------------------------------------------
// My original exploration of the HashSet / HashMap APIs
// (the polished version lives in src/ds/hashset.rs and src/ds/hashmap.rs)

pub fn basic_hashset_functions() {
    let mut hash: HashSet<String> = HashSet::new();

    // insert new items to the set
    hash.insert(String::from("foo"));
    hash.insert(String::from("bar"));
    hash.insert(String::from("dude"));

    // remove <&T> -> bool
    let remove = hash.remove("dude");

    // get <&T> -> Option<&T>
    let get = hash.get("foo");

    // (&self, <&T> ) -> bool
    let contains = hash.contains("dude");

    // keep only items that match
    hash.retain(|x| x == "bar");

    // drain all items
    hash.drain();

    // AWESOME EXTRACT WORKFLOW
    let mut set: HashSet<i32> = (0..8).collect();
    let extracted: HashSet<i32> = set.extract_if(|v| v % 2 == 0).collect();

    let mut evens: Vec<i32> = extracted.into_iter().collect();
    let mut odds: Vec<i32> = set.into_iter().collect();
    evens.sort();
    odds.sort();

    assert_eq!(evens, vec![0, 2, 4, 6]);
    assert_eq!(odds, vec![1, 3, 5, 7]);
}

pub fn basic_hashmap_functions() {
    let mut map: HashMap<String, i32> = HashMap::new();

    // insert returns an Option of old value if it exists
    map.insert(String::from("Doug"), 1);
    map.insert(String::from("Charles"), 1);
    map.insert(String::from("Wendy"), 1);
    map.insert(String::from("Tim"), 1);

    // remove by <K>, if exists, it returns Option<V>
    map.remove("Charles");

    // search for <&K> returns a bool
    let contains_key = map.contains_key("Doug");

    // just the dankest workflow. I love it so much.
    map.entry(String::from("Doug")).and_modify(|val| *val += 5).or_insert(1);

    // retain only values that match. uses key and value both!
    map.retain(|k, _| k == "Doug");
}
