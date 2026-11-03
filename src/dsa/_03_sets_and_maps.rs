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
pub fn basic_hash() {
    let mut hash: HashSet<String> = HashSet::new();

    hash.insert(String::from("foo"));
    hash.insert(String::from("bar"));

    if let Some(found) = hash.get("foo") {
        println!("foo was found! {}", found);
    } else {
        println!("bad news. foo not found.");
    }
}
