//! Backtracking. Notes live in book/src/patterns/backtracking.md.

// ANCHOR: subsets
/// 78. Subsets: at each index, two branches: skip it or take it.
pub fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    fn go(i: usize, nums: &[i32], path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            out.push(path.clone()); // clone: `path` keeps changing after this
            return;
        }
        go(i + 1, nums, path, out); //  skip nums[i]
        path.push(nums[i]); //          choose
        go(i + 1, nums, path, out); //  explore
        path.pop(); //                  un-choose
    }
    let mut out = Vec::new();
    go(0, nums, &mut Vec::new(), &mut out);
    out
}
// ANCHOR_END: subsets

// ANCHOR: permutations
/// 46. Permutations: try every unused element in the next slot.
pub fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    fn go(nums: &[i32], used: &mut [bool], path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            out.push(path.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            path.push(nums[i]);
            go(nums, used, path, out);
            path.pop();
            used[i] = false;
        }
    }
    let mut out = Vec::new();
    go(nums, &mut vec![false; nums.len()], &mut Vec::new(), &mut out);
    out
}
// ANCHOR_END: permutations

// ANCHOR: combination_sum
/// 39. Combination Sum: reuse allowed, so recurse with the SAME start index; prune early.
pub fn combination_sum(candidates: &[i32], target: i32) -> Vec<Vec<i32>> {
    fn go(start: usize, c: &[i32], remaining: i32, path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if remaining == 0 {
            out.push(path.clone());
            return;
        }
        for i in start..c.len() {
            if c[i] > remaining {
                break; // pruning: sorted input, so everything after is too big
            }
            path.push(c[i]);
            go(i, c, remaining - c[i], path, out);
            path.pop();
        }
    }
    let mut c = candidates.to_vec();
    c.sort_unstable();
    let mut out = Vec::new();
    go(0, &c, target, &mut Vec::new(), &mut out);
    out
}
// ANCHOR_END: combination_sum

#[test]
fn test_backtracking() {
    let mut s = subsets(&[1, 2, 3]);
    s.sort();
    assert_eq!(s.len(), 8);
    assert_eq!(s[0], Vec::<i32>::new());
    assert_eq!(permutations(&[1, 2, 3]).len(), 6);
    assert_eq!(permutations(&[1, 2])[1], [2, 1]);
    assert_eq!(combination_sum(&[2, 3, 6, 7], 7), vec![vec![2, 2, 3], vec![7]]);
}
