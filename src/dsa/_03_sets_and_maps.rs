//! ## Triggers
//! Sorted data, need to find some data inside, 0(n log n)
//!
//! ## Keywords
//! | dummy | dummy | dummy | dummy | dummy |
//! |---|---|---|---|---|
//!
//! ```
//! pub fn some_function()  {
//!     // something body of function
//!}
//! ```
//!
//! ## Most Important Functions
//! - insert(&mut self, value<T>) -> Option<T>
//! - get(&self, value: &T) -> Option<&T>
//! - Another Thing

use std::collections::HashSet;
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

use std::collections::HashMap;
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
