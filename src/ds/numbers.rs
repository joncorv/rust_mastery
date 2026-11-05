//! Integers, usize arithmetic, overflow and casting. These are where Rust interview code
//! usually panics.

#[test]
fn usize_pitfalls() {
    // ANCHOR: usize_pitfalls
    let len: usize = 0;
    // let last = len - 1;            // PANIC in debug: attempt to subtract with overflow
    let last = len.checked_sub(1); //   None
    let clamped = len.saturating_sub(1); // 0
    // Moving left from index i:
    let i: usize = 0;
    let left = i.checked_sub(1); //     None means "out of bounds", no negative index needed
    // Or do neighbor math in i32/isize and convert back after bounds-checking:
    let (r, dr): (usize, i32) = (0, -1);
    let nr = r as i32 + dr; //          -1, check `nr >= 0` before `as usize`
    let diff = 3usize.abs_diff(7); //   4, no underflow
    // ANCHOR_END: usize_pitfalls
    assert_eq!((last, clamped, left, nr, diff), (None, 0, None, -1, 4));
}

#[test]
fn overflow() {
    // ANCHOR: overflow
    let big = i32::MAX; //               2_147_483_647
    let wrapped = big.wrapping_add(1); //  i32::MIN
    let checked = big.checked_add(1); //   None
    let widened = big as i64 + 1; //       2_147_483_648, cast BEFORE the math
    // Binary-search midpoint without overflow:
    let (lo, hi) = (2_000_000_000i32, 2_100_000_000i32);
    let mid = lo + (hi - lo) / 2; //       not (lo + hi) / 2
    // Sentinels:
    let mut best = i32::MAX; //            "infinity" for a running min
    best = best.min(42);
    // ANCHOR_END: overflow
    assert_eq!((wrapped, checked, widened), (i32::MIN, None, 2_147_483_648));
    assert_eq!((mid, best), (2_050_000_000, 42));
}

#[test]
fn math() {
    // ANCHOR: math
    let a = (-7i32).abs(); //               7
    let p = 2i64.pow(10); //                1024
    let m = (-7i32).rem_euclid(3); //       2   (% gives -1 for negatives!)
    let r = -7 % 3; //                      -1
    let c = 7u32.div_ceil(2); //            4   ceiling division
    let s = 17u64.isqrt(); //               4   integer sqrt
    let (mn, mx) = (3.min(9), 3.max(9));
    let clamp = 15.clamp(0, 10); //         10
    let modded = (1_000_000_006i64 * 2) % 1_000_000_007; // the "answer mod 1e9+7" pattern
    // ANCHOR_END: math
    assert_eq!((a, p, m, r, c, s, mn, mx, clamp), (7, 1024, 2, -1, 4, 4, 3, 9, 10));
    assert_eq!(modded, 1_000_000_005);
}

#[test]
fn bits() {
    // ANCHOR: bits
    let x: u32 = 0b1011;
    let ones = x.count_ones(); //           3
    let lowest = x & x.wrapping_neg(); //   0b0001  lowest set bit
    let cleared = x & (x - 1); //           0b1010  drop the lowest set bit
    let is_pow2 = 16u32.is_power_of_two(); // true
    let bit2 = (x >> 2) & 1; //             0  test bit i
    let set3 = x | (1 << 2); //             0b1111  set bit i
    let xor_all = [4, 1, 2, 1, 2].iter().fold(0, |a, b| a ^ b); // 4: "single number"
    let tz = 8u32.trailing_zeros(); //      3
    // ANCHOR_END: bits
    assert_eq!((ones, lowest, cleared, is_pow2, bit2, set3, xor_all, tz), (3, 1, 0b1010, true, 0, 0b1111, 4, 3));
}
