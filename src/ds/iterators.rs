//! Iterator adapters: the Rust interview superpower.

#[test]
fn iter_kinds() {
    // ANCHOR: iter_kinds
    let mut v = vec![1, 2, 3];
    for x in v.iter() { /* x: &i32,     borrow   */ }
    for x in v.iter_mut() {
        *x *= 10; //          x: &mut i32, mutate in place
    }
    for x in v.clone().into_iter() { /* x: i32,      consumes the Vec */ }
    // `for x in &v` == v.iter(),  `for x in &mut v` == v.iter_mut(),  `for x in v` == into_iter()
    // ANCHOR_END: iter_kinds
    assert_eq!(v, [10, 20, 30]);
}

#[test]
fn transform() {
    // ANCHOR: transform
    let v = vec![1, 2, 3, 4, 5, 6];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    let evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let parsed: Vec<i32> = ["1", "x", "3"].iter().filter_map(|s| s.parse().ok()).collect(); // [1, 3]
    let flat: Vec<i32> = vec![vec![1], vec![2, 3]].into_iter().flatten().collect();
    let pairs: Vec<(usize, char)> = "ab".chars().enumerate().collect(); // [(0,'a'), (1,'b')]
    let zipped: Vec<(i32, char)> = v.iter().copied().zip("xyz".chars()).collect(); // stops at shorter
    let (small, big): (Vec<i32>, Vec<i32>) = v.iter().partition(|&&x| x < 4);
    // ANCHOR_END: transform
    assert_eq!(doubled, [2, 4, 6, 8, 10, 12]);
    assert_eq!(evens, [2, 4, 6]);
    assert_eq!(parsed, [1, 3]);
    assert_eq!(flat, [1, 2, 3]);
    assert_eq!(pairs, [(0, 'a'), (1, 'b')]);
    assert_eq!(zipped, [(1, 'x'), (2, 'y'), (3, 'z')]);
    assert_eq!((small, big), (vec![1, 2, 3], vec![4, 5, 6]));
}

#[test]
fn slicing_iters() {
    // ANCHOR: slicing_iters
    let v = vec![1, 2, 3, 4, 5, 6];
    let a: Vec<_> = v.iter().skip(1).step_by(2).collect(); //         [2, 4, 6]
    let b: Vec<_> = v.iter().take_while(|&&x| x < 3).collect(); //    [1, 2]
    let c: Vec<_> = v.iter().rev().take(2).collect(); //              [6, 5]
    let d: Vec<_> = v.iter().skip_while(|&&x| x < 5).collect(); //    [5, 6]
    let e: Vec<_> = (0..10).rev().step_by(3).collect(); //            [9, 6, 3, 0]
    // ANCHOR_END: slicing_iters
    assert_eq!(a, [&2, &4, &6]);
    assert_eq!(b, [&1, &2]);
    assert_eq!(c, [&6, &5]);
    assert_eq!(d, [&5, &6]);
    assert_eq!(e, [9, 6, 3, 0]);
}

#[test]
fn reduce() {
    // ANCHOR: reduce
    let v = vec![3, 1, 4, 1, 5];
    let sum: i32 = v.iter().sum(); //                      14 (annotate the type!)
    let prod: i64 = v.iter().map(|&x| x as i64).product(); // 60
    let cnt = v.iter().filter(|&&x| x == 1).count(); //    2
    let max = v.iter().max(); //                           Some(&5)
    let argmax = v.iter().enumerate().max_by_key(|&(_, x)| x).map(|(i, _)| i); // Some(4)
    let any_big = v.iter().any(|&x| x > 4); //             true
    let all_pos = v.iter().all(|&x| x > 0); //             true
    let found = v.iter().find(|&&x| x > 3); //             Some(&4)
    let folded = v.iter().fold(0, |acc, x| acc * 10 + x); // 31415
    // running prefix sums
    let prefix: Vec<i32> = v
        .iter()
        .scan(0, |s, &x| {
            *s += x;
            Some(*s)
        })
        .collect(); //                                       [3, 4, 8, 9, 14]
    // ANCHOR_END: reduce
    assert_eq!((sum, prod, cnt, max, argmax), (14, 60, 2, Some(&5), Some(4)));
    assert!(any_big && all_pos);
    assert_eq!((found, folded), (Some(&4), 31415));
    assert_eq!(prefix, [3, 4, 8, 9, 14]);
}

#[test]
fn max_by_gotchas() {
    // ANCHOR: max_by_gotchas
    // max_by_key returns the LAST max on ties, min_by_key returns the FIRST min.
    let v = [(1, 'a'), (3, 'b'), (3, 'c')];
    let last_max = v.iter().max_by_key(|p| p.0); //      Some(&(3, 'c'))
    // f64 isn't Ord, so max() won't compile. Use fold or max_by with total_cmp:
    let fs: [f64; 3] = [1.5, -2.0, 9.25];
    let fmax = fs.iter().copied().fold(f64::MIN, f64::max); // 9.25
    let fmax2 = fs.iter().max_by(|a, b| a.total_cmp(b)); //    Some(&9.25)
    // ANCHOR_END: max_by_gotchas
    assert_eq!(last_max, Some(&(3, 'c')));
    assert_eq!((fmax, fmax2), (9.25, Some(&9.25)));
}

#[test]
fn peekable() {
    // ANCHOR: peekable
    // Run-length encoding with a peekable iterator: "aaabcc" -> "a3b1c2"
    let s = "aaabcc";
    let mut out = String::new();
    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        let mut n = 1;
        while it.next_if_eq(&c).is_some() {
            n += 1;
        }
        out.push_str(&format!("{c}{n}"));
    }
    // slices have chunk_by for the same thing:
    let groups: Vec<&[i32]> = [1, 1, 2, 3, 3].chunk_by(|a, b| a == b).collect();
    // ANCHOR_END: peekable
    assert_eq!(out, "a3b1c2");
    assert_eq!(groups, vec![&[1, 1][..], &[2], &[3, 3]]);
}
