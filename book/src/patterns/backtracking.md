# Backtracking

## Triggers

- "Return **all** subsets / permutations / combinations / partitions / boards."
- Constraint puzzles: N-Queens, Sudoku, word search.
- The input is **small** (n ≤ ~15–20). That's a hint that exponential time is expected.

## Keywords

| all possible | generate | combinations | permutations | subsets |
|---|---|---|---|---|

## The big idea

Build a solution **one choice at a time**, recurse, then **undo** the choice:

```text
fn go(state):
    if state is complete: record a copy; return
    for each choice:
        if not valid: continue      // pruning
        choose
        go(state)
        un-choose
```

| Kind | Branching | Size |
|---|---|---|
| subsets | take / skip each element | 2ⁿ |
| combinations (k of n) | pick from `start..`, recurse with `i + 1` | C(n, k) |
| permutations | any unused element | n! |
| reuse allowed | recurse with the same `i` | depends on the target |

## Rust toolkit

- A nested `fn go(..., path: &mut Vec<T>, out: &mut Vec<Vec<T>>)`. Pass everything explicitly.
- `path.push(x); go(..); path.pop();`: choose / explore / un-choose.
- `out.push(path.clone())`: record a **copy**.
- `used: &mut [bool]` for permutations. Sort the input first to prune or skip duplicates.

## Template: subsets (take / skip)

```rust
{{#include ../../../src/dsa/_11_backtracking.rs:subsets}}
```

## Template: permutations

```rust
{{#include ../../../src/dsa/_11_backtracking.rs:permutations}}
```

## Template: combinations with pruning

```rust
{{#include ../../../src/dsa/_11_backtracking.rs:combination_sum}}
```

## Rust gotchas

- **Forgetting `.clone()`** when recording: `out.push(path)` would move `path`, which won't
  compile. That's Rust catching the classic "all results are the same list" bug for you.
- **Forgetting to un-choose** (`pop`, `used[i] = false`) gives wrong answers, not a compile error.
- **Closures can't recurse.** Use a nested `fn`.
- **Duplicates in input** (Subsets II, Permutations II): sort, then
  `if i > start && nums[i] == nums[i - 1] { continue }`.

## Practice

- [ ] [Subsets (78)](https://leetcode.com/problems/subsets/)
- [ ] [Permutations (46)](https://leetcode.com/problems/permutations/)
- [ ] [Combinations (77)](https://leetcode.com/problems/combinations/)
- [ ] [Combination Sum (39)](https://leetcode.com/problems/combination-sum/)
- [ ] [Letter Combinations of a Phone Number (17)](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)
- [ ] [Word Search (79)](https://leetcode.com/problems/word-search/)
- [ ] [N-Queens (51)](https://leetcode.com/problems/n-queens/)
