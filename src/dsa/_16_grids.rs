//! Grids. Notes live in book/src/patterns/grids.md.

// ANCHOR: neighbors
pub const DIRS4: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// In-bounds neighbors of (r, c). checked_add_signed returns None below 0, so there's
/// no usize underflow and no casting back and forth.
pub fn neighbors(r: usize, c: usize, rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
    DIRS4.iter().filter_map(move |&(dr, dc)| {
        let nr = r.checked_add_signed(dr)?;
        let nc = c.checked_add_signed(dc)?;
        (nr < rows && nc < cols).then_some((nr, nc))
    })
}
// ANCHOR_END: neighbors

// ANCHOR: num_islands
/// 200. Number of Islands: flood fill, overwriting the grid to mark cells visited.
pub fn num_islands(grid: &mut [Vec<char>]) -> usize {
    let (rows, cols) = (grid.len(), grid.first().map_or(0, |row| row.len()));
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '1' {
                continue;
            }
            count += 1;
            grid[r][c] = '0';
            let mut stack = vec![(r, c)];
            while let Some((cr, cc)) = stack.pop() {
                for (nr, nc) in neighbors(cr, cc, rows, cols) {
                    if grid[nr][nc] == '1' {
                        grid[nr][nc] = '0';
                        stack.push((nr, nc));
                    }
                }
            }
        }
    }
    count
}
// ANCHOR_END: num_islands

// ANCHOR: rotate
/// 48. Rotate Image 90° clockwise in place = transpose, then reverse each row.
pub fn rotate(m: &mut [Vec<i32>]) {
    let n = m.len();
    for i in 0..n {
        for j in i + 1..n {
            // m[i][j] and m[j][i] are in different rows, but the borrow checker sees two &mut m.
            // Copy types make a temp swap the easy way out.
            let tmp = m[i][j];
            m[i][j] = m[j][i];
            m[j][i] = tmp;
        }
    }
    for row in m.iter_mut() {
        row.reverse();
    }
}
// ANCHOR_END: rotate

#[test]
fn test_grids() {
    let mut n: Vec<_> = neighbors(0, 0, 3, 3).collect();
    n.sort();
    assert_eq!(n, [(0, 1), (1, 0)]);
    assert_eq!(neighbors(1, 1, 3, 3).count(), 4);

    let mut grid: Vec<Vec<char>> = ["11000", "11000", "00100", "00011"].iter().map(|r| r.chars().collect()).collect();
    assert_eq!(num_islands(&mut grid), 3);
    assert_eq!(num_islands(&mut []), 0);

    let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut m);
    assert_eq!(m, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
}
