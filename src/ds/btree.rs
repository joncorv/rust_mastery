//! BTreeMap / BTreeSet: sorted by key, O(log n) operations. Reach for these when you
//! need "smallest/largest key", "next key above x", or sorted iteration.

use std::collections::{BTreeMap, BTreeSet};

#[test]
fn btreemap() {
    // ANCHOR: btreemap
    let mut m = BTreeMap::new();
    for (k, v) in [(30, "c"), (10, "a"), (20, "b")] {
        m.insert(k, v);
    }
    let keys: Vec<i32> = m.keys().copied().collect(); //   [10, 20, 30] always sorted
    // .map(..) copies out of the borrow so we can mutate `m` afterwards
    let min = m.first_key_value().map(|(&k, &v)| (k, v)); // Some((10, "a"))
    let max = m.last_key_value().map(|(&k, &v)| (k, v)); //  Some((30, "c"))
    let popped = m.pop_first(); //                         Some((10, "a")), removes it
    // Same API as HashMap otherwise: get, insert, remove, entry, contains_key, ...
    // ANCHOR_END: btreemap
    assert_eq!(keys, [10, 20, 30]);
    assert_eq!((min, max), (Some((10, "a")), Some((30, "c"))));
    assert_eq!(popped, Some((10, "a")));
}

#[test]
fn range_queries() {
    // ANCHOR: range_queries
    let set: BTreeSet<i32> = [1, 5, 9, 13].into();
    // floor / ceiling: largest <= x, smallest >= x
    let floor_7 = set.range(..=7).next_back(); //  Some(&5)
    let ceil_7 = set.range(7..).next(); //         Some(&9)
    let between: Vec<i32> = set.range(2..10).copied().collect(); // [5, 9]
    let (lo, hi) = (set.first(), set.last()); //   Some(&1), Some(&13)
    // ANCHOR_END: range_queries
    assert_eq!((floor_7, ceil_7), (Some(&5), Some(&9)));
    assert_eq!(between, [5, 9]);
    assert_eq!((lo, hi), (Some(&1), Some(&13)));
}

#[test]
fn multiset() {
    // ANCHOR: multiset
    // Rust has no multiset. Use BTreeMap<value, count>.
    let mut ms: BTreeMap<i32, usize> = BTreeMap::new();
    for x in [3, 1, 3, 2] {
        *ms.entry(x).or_insert(0) += 1;
    }
    // remove ONE copy of 3
    if let Some(c) = ms.get_mut(&3) {
        *c -= 1;
        if *c == 0 {
            ms.remove(&3);
        }
    }
    let smallest = *ms.keys().next().unwrap(); // 1
    // ANCHOR_END: multiset
    assert_eq!(ms[&3], 1);
    assert_eq!(smallest, 1);
}
