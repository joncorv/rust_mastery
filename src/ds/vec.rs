//! Vec<T> and slices (&[T]). Everything here is checked by `cargo test`.
//! The book includes these blocks by ANCHOR name.

#[test]
fn create() {
    // ANCHOR: create
    let empty: Vec<i32> = Vec::new();
    let with_cap: Vec<i32> = Vec::with_capacity(100); // no reallocs until len 100
    let filled = vec![0; 5]; // [0, 0, 0, 0, 0]
    let grid = vec![vec![0; 3]; 2]; // 2 rows x 3 cols
    let from_range: Vec<i32> = (1..=4).collect(); // [1, 2, 3, 4]
    let from_str: Vec<char> = "abc".chars().collect(); // ['a', 'b', 'c']
    // ANCHOR_END: create
    assert!(empty.is_empty() && with_cap.capacity() >= 100);
    assert_eq!(filled, [0, 0, 0, 0, 0]);
    assert_eq!(grid, [[0, 0, 0], [0, 0, 0]]);
    assert_eq!(from_range, [1, 2, 3, 4]);
    assert_eq!(from_str, ['a', 'b', 'c']);
}

#[test]
fn add_remove() {
    // ANCHOR: add_remove
    let mut v = vec![1, 2, 3];
    v.push(4); //                          O(1)  [1, 2, 3, 4]
    let last = v.pop(); //                 O(1)  Some(4)
    v.insert(0, 9); //                     O(n)  [9, 1, 2, 3]
    let removed = v.remove(0); //          O(n)  9, keeps order
    let swapped = v.swap_remove(0); //     O(1)  1, last elem fills the hole -> [3, 2]
    v.extend([7, 8]); //                         [3, 2, 7, 8]
    v.truncate(2); //                            [3, 2]
    v.clear(); //                                []
    // ANCHOR_END: add_remove
    assert_eq!((last, removed, swapped), (Some(4), 9, 1));
    assert!(v.is_empty());
}

#[test]
fn access() {
    // ANCHOR: access
    let v = vec![10, 20, 30];
    let a = v[0]; //                 panics if out of bounds
    let b = v.get(5); //             None instead of panic
    let first = v.first(); //        Some(&10)
    let last = v.last(); //          Some(&30)
    let n = v.len();
    // `last index` is v.len() - 1, which UNDERFLOWS (panics) on an empty Vec!
    let safe_last_idx = v.len().checked_sub(1); // Some(2)
    // ANCHOR_END: access
    assert_eq!((a, b, first, last, n, safe_last_idx), (10, None, Some(&10), Some(&30), 3, Some(2)));
}

#[test]
fn in_place() {
    // ANCHOR: in_place
    let mut v = vec![3, 1, 2, 2, 5];
    v.swap(0, 4); //                     [5, 1, 2, 2, 3]
    v.reverse(); //                      [3, 2, 2, 1, 5]
    v.sort(); //                         [1, 2, 2, 3, 5]  stable, O(n log n)
    v.dedup(); //                        [1, 2, 3, 5]     removes CONSECUTIVE dups only
    v.retain(|&x| x != 3); //            [1, 2, 5]
    v.rotate_left(1); //                 [2, 5, 1]
    v.fill(0); //                        [0, 0, 0]
    // ANCHOR_END: in_place
    assert_eq!(v, [0, 0, 0]);
}

#[test]
fn search() {
    // ANCHOR: search
    let v = vec![1, 3, 5, 7, 9];
    let has = v.contains(&5); //                         O(n) true
    let pos = v.iter().position(|&x| x == 7); //         O(n) Some(3)
    let found = v.binary_search(&7); //                  O(log n) Ok(3)
    let missing = v.binary_search(&4); //                Err(2) = where 4 WOULD be inserted
    // first index where the predicate turns false (v must be partitioned by it)
    let first_ge_6 = v.partition_point(|&x| x < 6); //   3   (aka lower_bound)
    // ANCHOR_END: search
    assert_eq!((has, pos, found, missing, first_ge_6), (true, Some(3), Ok(3), Err(2), 3));
}

#[test]
fn slices() {
    // ANCHOR: slices
    let v = vec![1, 2, 3, 4, 5];
    let mid = &v[1..4]; //                    [2, 3, 4]
    let (left, right) = v.split_at(2); //     [1, 2] and [3, 4, 5]
    let pairs: Vec<i32> = v.windows(2).map(|w| w[1] - w[0]).collect(); // [1, 1, 1, 1]
    let chunks: Vec<&[i32]> = v.chunks(2).collect(); // [[1,2],[3,4],[5]]
    let total: i32 = v.iter().sum();
    let max = v.iter().max(); //              Some(&5)
    let joined = [vec![1], vec![2, 3]].concat(); // [1, 2, 3]
    let sorted = v.is_sorted();
    // ANCHOR_END: slices
    assert_eq!(mid, [2, 3, 4]);
    assert_eq!((left, right), (&[1, 2][..], &[3, 4, 5][..]));
    assert_eq!(pairs, [1, 1, 1, 1]);
    assert_eq!(chunks, vec![&[1, 2][..], &[3, 4], &[5]]);
    assert_eq!((total, max, sorted), (15, Some(&5), true));
    assert_eq!(joined, [1, 2, 3]);
}

#[test]
fn borrow_two() {
    // ANCHOR: borrow_two
    let mut v = vec![1, 2, 3, 4];
    // let a = &mut v[0]; let b = &mut v[3];   // ERROR: two mutable borrows of `v`
    v.swap(0, 3); //                             1) if you just need to swap
    let (lo, hi) = v.split_at_mut(2); //         2) split into two disjoint &mut slices
    lo[0] += hi[1];
    if let Ok([a, b]) = v.get_disjoint_mut([0, 3]) {
        // 3) disjoint &mut by index (Rust 1.86+)
        std::mem::swap(a, b);
    }
    // ANCHOR_END: borrow_two
    assert_eq!(v, [1, 2, 3, 5]);
}

#[test]
fn grid_2d() {
    // ANCHOR: grid_2d
    let mut grid = vec![vec!['.'; 4]; 3]; // 3 rows, 4 cols
    let (rows, cols) = (grid.len(), grid[0].len());
    grid[1][2] = '#';
    // transpose
    let t: Vec<Vec<char>> = (0..cols).map(|c| (0..rows).map(|r| grid[r][c]).collect()).collect();
    // ANCHOR_END: grid_2d
    assert_eq!((t.len(), t[0].len()), (4, 3));
    assert_eq!(t[2][1], '#');
}
