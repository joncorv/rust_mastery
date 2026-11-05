# Sets & Maps

## Triggers

- "Have I **seen** this before?" (duplicates, cycles, complements).
- **Count** things: frequencies, majority element, "most common".
- **Group** things by a computed key (anagrams, same digit sum, same domain).
- The naive solution is a nested loop that searches an array. A hash lookup turns
  **O(n²) into O(n)**.

## Keywords

| duplicate | unique / distinct | frequency / count | anagram | group by |
|---|---|---|---|---|

## The big idea

Trade **O(n) memory for O(1) lookups**. Ask: *what would I need to look up instantly to
avoid the inner loop?* That thing is the key.

| Need | Use |
|---|---|
| membership only | `HashSet<T>` |
| key → count / index / list | `HashMap<K, V>` + the Entry API |
| keys are `a`–`z` | `[i32; 26]` indexed by `(b - b'a') as usize`, faster and hashable |
| keys must stay sorted | `BTreeMap` / `BTreeSet` |

## Rust toolkit

- `*map.entry(k).or_insert(0) += 1` for counting, `.or_default().push(x)` for grouping.
- `set.insert(x)` returns `false` if `x` was already there, so you get "check + insert" in one call.
- Arrays and tuples are `Hash`: `HashMap<[u8; 26], _>` makes a great anagram key.
- See the [HashMap](../cheatsheet/hashmap.md) and [HashSet](../cheatsheet/hashset.md) cheat sheets.

## Template: counting with an array

```rust
{{#include ../../../src/dsa/_03_sets_and_maps.rs:is_anagram}}
```

## Template: group by key

```rust
{{#include ../../../src/dsa/_03_sets_and_maps.rs:group_anagrams}}
```

## Template: set lookups instead of sorting

```rust
{{#include ../../../src/dsa/_03_sets_and_maps.rs:longest_consecutive}}
```

## Rust gotchas

- **Iteration order is random** and changes between runs. Sort before comparing output in tests.
- **`map[&k]` panics on a missing key.** Use `get`, `get(..).copied().unwrap_or(0)`, or `entry`.
- **`f64` can't be a key** (it isn't `Hash`/`Eq`). Use integers or `to_bits()`.
- **`String` keys with `&str` lookups work** (`map.get("doug")`) thanks to `Borrow`, so you
  don't have to allocate to look something up.
- **Reading and inserting in the same expression** fails the borrow check
  (`map.insert(k, map[&j])`). Copy the value into a local first.

## Practice

- [ ] [Contains Duplicate (217)](https://leetcode.com/problems/contains-duplicate/)
- [ ] [Two Sum (1)](https://leetcode.com/problems/two-sum/)
- [ ] [Valid Anagram (242)](https://leetcode.com/problems/valid-anagram/)
- [ ] [Group Anagrams (49)](https://leetcode.com/problems/group-anagrams/)
- [ ] [Top K Frequent Elements (347)](https://leetcode.com/problems/top-k-frequent-elements/)
- [ ] [Longest Consecutive Sequence (128)](https://leetcode.com/problems/longest-consecutive-sequence/)
- [ ] [Isomorphic Strings (205)](https://leetcode.com/problems/isomorphic-strings/)
